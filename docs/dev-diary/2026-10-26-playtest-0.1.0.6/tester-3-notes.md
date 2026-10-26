# Playtester 3 notes (hardcore 4X / grand strategy persona)

## Highlights so far
- FOUR WINS FROM FOUR, all first plays: Archivists T23, Custodians T29, Prospectors T29, Arkwrights T21. No AI rival passed 52% Victory in any game.
- The "weak" Factions won FASTEST in my hands (Arkwrights T21, Archivists T23). Their low win rates are an AI problem: AI Arkwrights had 0, 0 and 5 Colonists beyond Earth orbit, dumping Pioneers in Antarctica, and AI Archivists never researched The Upload in G2-G4.
- EXPLOIT: `build module-ducats` costs 2 Ducats per Material, needs no Widgets and lands next turn. It is cheaper than market Materials and the real build button off Earth.
- EXPLOIT: a Trade Post at a 20+ colony paid +60 Ducats a turn (G4 Moon). An Observatory over Earth (R 2.00) makes 7-8 Research against a Lab's 1-2.
- The best game: Custodians. Scrubbers raced a table whose emissions kept climbing, and the pack nearly bought China with my six Scrubbers in it. Taking a neutral Region raises world emissions, a great hidden trade-off that the text never states.
- The Prospectors' Fund compounds 1% per payer per turn; half of 6500 came in the last 3 turns. Their expansion pushed the collapse projection to T25 by turn 11, the one time the climate forced my hand.
- I won the Moon race 4 of 4 (T5-T8); in four games the AI built one warship and never fought.
- CLI friction: a Refuel drains Fuel and blacks out Power Plants (5 times); no Shortfall forecast or emissions by source; offers appear only when `turn` refuses; the raw key "[exodus_call_order]" leaks into the report.
- Rating 6/10: rich interlocking systems, but solved fast against a passive AI.

## Game 1: seed 1301, Archivists (start China)

Pre-game: read README, the whole GLOSSARY and the `help` grammar.
- LOST/FRUSTRATED: GLOSSARY.md is 224 KB, and most of it is version history ("since version 0.09.4 ... until 0.1.0.3 ..."). To find the current rule I have to strip out ten earlier ones. Example: the Archive entry carries 7 version notes. As a rules reader I want the current rule first and the history in a footnote, or cut entirely.
- LOST: nothing in text play tells me my Faction's multipliers (Research xN, Influence xN). The Faction card exists in the window; `show` never prints it.

T1: Tech LLM (Society chain to The Upload). Research Lab in China, Shipyard on Axiom, 2 Pioneers, 13 Influence on Japan (threshold 30, cheapest neighbour).
T2: LLM done in one turn (world makes about 18 a turn). I led, +5 Influence. Picked Civil Defense. Lifted 2 to Axiom, bought 3 Influence, all 21 on Japan. EXCITED: Archivists +5 Influence per Tech led is a nice engine.
T2: all three AIs offered non-aggression; the turn would not end until I answered. Accepted all. The offers are not in `show` before you run `turn`, so I only found them in the refusal. FRUSTRATED (text tool).
T3: Japan mine (taken by Influence on turn 2). Civil Defense done; picked Orbital Data Centers. Observatory on Axiom, Influence 19 on India.
  - Odd text: ISS card reads "building: 0:Shipyard 8/10w (the Custodians's, cancellable)". The possessive is wrong, and I cannot cancel a rival's build at a place I don't direct.
T4: ODC done (another 32-Research Tech in one turn). Superconductors picked. Bought Materials, Fuel and 7 Influence; 26 on India. Colony Ship ordered at Axiom.
  - FRUSTRATED: the Colony Ship queued BEHIND the Observatory at Axiom's 4 Widgets a turn, so it took 3 turns. A station's Core makes 4 Widgets; everything off Earth crawls.
  - FRUSTRATED: I spent my Fuel down to 0.2 on the Colony Ship's tank. Next Income my Power Plants and Accumulator ran at a quarter (Energy -8.2). The text tool's "After them: 0.2F" line could have warned me. The window forecasts a Shortfall; the CLI doesn't.
T5: India taken T4. Coral Die-off break, +1 Unrest everywhere. Research Lab India, 15 Influence on Indonesia.
T6: Observatory online: +7 Research, against +1.1 from the India Lab. EXPLOIT-ish: an Observatory over Earth (R 2.00) is about six times a Lab for the same Materials. Picked The Upload. Renewed all three Accords.
T7: The Upload complete on turn 6. Victory 8% (the gate's 6% plus 10 fund from the Opening Objective). Colony Ship built. Picked Expanded Habitats. Loaded 4 from China and flew to the Moon in one turn (15.7 Fuel). Mine in China.
T8: Indonesia taken. Permafrost break; Antarctica open. Collapse projection: turn 26. Unloaded at Mare Imbrium.
T9: WON THE MOON RACE. EXCITED. +5 Influence, +1 a turn, -0.5 Unrest everywhere. The Moment lists where each rival stood ("Custodians were in its orbit"). The Custodians' Colony Ship was in lunar orbit the same turn; I beat them by a turn. Good drama.
  - Three Techs completed in one Income (Deep Mining, Efficient Grids, Automated Refining). I picked one; the AIs apparently picked the others.
  - LOST: the Research block says "Next pick: Arkwrights." while also demanding that *I* pick now. I think it means who leads the race for the Tech after this one. Unclear.
  - Collapse projection: turn 25. "Last turn to act: 22." I emit the most (22.7 of 72 ppm).
T10: Vietnam taken. Moon colony founded on T8 (unloaded), 4 Colonists. EXPLOIT: `build module-ducats` costs 2 Ducats per Material of the price (Habitat 27.5M = 55 Ducats). It ignores Widgets and completes at the next Resolution. Buying the Materials at market would cost 3 Ducats each AND still need the Widgets. On a 4-Widget colony this is the real build button. Repro: any colony; `build module-ducats <colony> habitat`.
  - Set `fund-archive` (100%) at 40 Research a turn. Mothballed the Power Plants in Japan and India to cut emissions. Each Mothball costs +1 Unrest.
T11: SEA ROSE at +1.8 C and took six of my Facilities, among them my Unique Accumulator. FRUSTRATED but fair. It is a real climate cost, and I am the main polluter.
  - FRUSTRATED: `build module mareimbrium mine` was refused "full until more Colonists live here" while the same order list unloads 2 Colonists there. The slot check ignores Colonists landing this turn, so every Moon build lags a turn behind the ferry.
  - The Custodians let our Accord lapse at once (they hold Blame against me: I am at 38%).
T12: Fuel ran short AGAIN (I spent it on a tank refill). Generator and Power Plant at a quarter. FRUSTRATED: the CLI has no Shortfall forecast; the window does.
  - Replaced both mothballed Power Plants with Wind Farms for 12.5 Materials each, done in one turn. Good, cheap lever.
T13: Australia taken. The Sink Weakens break. Archive fund 113/250 at 43-45 Research a turn.
T14: Kazakhstan taken. Two Colony Ships now. The Colony Ship again queued behind a Heliostat at Axiom's 4 Widgets a turn.
T15: Moon at 12 Colonists, upgraded to Settlement. Victory 37%.
T16: Archive FUND FULL on turn 16. Ethiopia taken. Bought three Moon Modules outright with Ducats, two by Widgets, one by Materials.
  - The Prospectors and Arkwrights "will not renew" our Accords; I am Cold or Wary with half the table. Blame 41%.
T17: First child born off Earth (+5 Influence), at my Moon colony. Ice Sheets Committed break; the sea took six more of my Facilities.
T18: Moon at 20 Colonists and 12 Modules: upgraded to Colony tier. Mexico taken.
T19: Archive ordered (27.5 Materials and 11 Widgets after Build Where You Dig and Regolith Printing). Complete the same Resolution: "Research paid in full".
T20-T23: uploaded 4 a turn. From 72% Victory the pack started spending Influence on China, nowhere near enough. On T22 the Arkwrights attacked my Ethiopia with three Armies and destroyed its Army (EXCITED: the AI finally lashed out). Too late.
**T23: ARCHIVISTS WIN, turn 23 of 50.** Victory 100%. Rivals at the end: Custodians 38% (12/20 off Earth, never a Stabilization phase), Prospectors 31% (Fund 816/6500), Arkwrights 6% (ZERO Colonists beyond Earth orbit on turn 23, held for want of a Colonial Capital).

### Game 1 verdict
- A solved line, found on the first try with no prior knowledge: Lab, Observatory over Earth, Shipyard, Colony Ship to the Moon by T8, Influence on a cheap neighbour every 2-3 turns, fund-archive at about 40 Research a turn from T10, ferry 6-7 a trip, buy the last Modules with Ducats, Colony tier T18, Archive T19, upload T20-23.
- The tension designed in (lead Techs for Influence, or fund the Archive) barely bit. The AIs research fast, so The Upload's whole chain fell in 6 turns while I led, and I then switched to 100% fund with no gate left to wait for.
- The climate never caught me: the collapse projection swung T25 > T29 > T39 as Green Consensus and the Sink's growth came in. I was the dirtiest seat (Blame 45%) and paid for it only in Relations and thresholds x1.2, never in a way that stopped me.
- AI: never contested a single Region I climbed (Japan, India, Indonesia, Vietnam, Australia, Kazakhstan, Ethiopia, Mexico, Argentina, Brazil: ten Regions by Influence, uncontested). The pack mechanic woke at 72% and spent 5-30 Influence a turn on a Home Region whose price was 85-117.

## Game 2: seed 1302, Custodians (start China)

T1: Efficient Grids. Scrubber in China (33 M): +3 ppm on the Sink, the Opening Objective met at once (+0.5 for good). Shipyard on ISS, 14 Influence on Japan.
T2: same three non-aggression offers from the AI bloc; accepted.
T3: Japan taken. Solar Array on ISS.
T5: Green Consensus picked (per-person x0.5 AND thresholds x0.75). For a Custodian that is the best Tech in the game.
T6: India taken (Green Consensus cut its price). Colony Ship.
T7-T8: WON THE MOON RACE again, by one turn over the Prospectors' loaded ship. EXCITED, but twice out of two it was easy.
  - FRUSTRATED: the Prospectors' Trade offer surfaced only when `turn` refused. I could not read it beforehand with `show`.
T8: Leapfrog China. Indonesia taken.
T9-T10: Clean Manufacturing. LOST: I tried a station at Earth L4 (it counts as beyond Earth orbit). I sent 3 Colonists, and a station cannot be founded with fewer than 4. Nothing warned me at the change-orbit (16.3 Fuel); the ship sat useless at L4 for two turns. FRUSTRATED.
  - LOST: `transit magellan earth 5` is refused "already there"; a far orbit is reached by `change-orbit`, though the glossary calls it "a place of its own, priced as any journey is". The help text says change-orbit costs "1 Fuel", but to L4 it charged 16.3.
T11: EMISSIONS ROSE when I took Vietnam (51.7 to 61). A neutral Region's Facilities are idle and emit to nobody; take it and they start emitting. For a Custodian, expanding on Earth is self-harm unless you mothball what you take. Clever and non-obvious, a real decision. EXCITED (once I worked it out). LOST, because no line says so: I only found it by comparing the totals.
  - LOST: the CLI never shows the Climate Panel's by-source breakdown, only "emissions by seat". I could not tell what caused a +9 ppm jump.
T12-T13: Fuel ran short TWICE more. A refuel drained the Stockpile to 0, and my Power Plants and the Moon Generator ran at a quarter (-31.8 Energy one turn). FRUSTRATED, and this is the third game-turn this has bitten me. Refuel should warn, or keep back the Power Plants' upkeep.
T14: the sea took 7 Facilities (+1.8 C).
T15: "[exodus_call_order]" appeared in the Rival report. A raw string key is leaking into player text (seen again T21). BUG.
T16: Planetary Stewardship (my gate) done. Leapfrog India. Accepted an Archivist trade: 30 Energy for my 20 Fuel.
T17: Second Objective (3 Leapfroggs) gave 25 Influence; Allotment 47. Then I had nothing worth buying with Influence. More Regions add emissions; rivals' places were behind non-aggression. BORED/LOST: a Custodian's Influence has no good sink mid-game.
T17: set the carbon-credit offer to 30 ppm a turn. The Arkwrights and Prospectors bought 10 each at 1-1.5 Ducats a ppm: a few Ducats a turn, no real effect.
T18-T20: Earth L4 founded (4), Habitat on it bought with Ducats. Mothballed every Earth Power Plant: my Energy came from a Moon Generator at x2.08 (Production Moved, an idle Earth Power Plant paired with it) and x3 (a Helium-3 card): +70 Energy from one Module. EXPLOIT-ish: Production Moved plus the Moon Generator's 1.50 yield is a huge Energy source for a Custodian. Repro: mothball any Earth Power Plant, have a Generator off Earth.
T20: a Labour Dispute in India shut all 6 of its Scrubbers for a turn: the Sink fell from 37 to 25 in one card. A single Event can break a Stabilization run; the run's exposure to one card felt harsh. FRUSTRATED.
T20: refused the Whistleblower (-3 Relations), and all three rivals let their Accords lapse the next turn.
T22: 20 Colonists beyond Earth orbit met (Moon 12, Earth L4 8). Victory 63%.
T22-T25: THE TREADMILL. Each scrubber I added (+3 ppm), the AIs added back: Prospectors' Strip Permits and Industry Levels, the Arkwrights taking Mexico. Emissions 50.7 > 54 > 58.6 > 61 > 62. EXCITED: this was the best stretch of the four games so far, a real race against the table's emissions.
T26: first phase under the Sink (-0.84). Energy -40 a turn, so Wind Farm replacements (12.5 M) and mothballed Labs and the Shipyard.
T27: THE PACK: the Arkwrights spent 30 on China ("China could fall to Arkwrights next turn"), with my six Scrubbers in it. Bought 10 Influence and put 37 on China. EXCITED: real threat, real answer. Good warning line.
T28: the Arkwrights Agitated China and Nigeria; the Prospectors marched an Army into Nigeria (73% to hold the field) and failed.
**T29: CUSTODIANS WIN, turn 29.** Run 4 of 4 (net -0.8, -5.2, -6.8, -12.7), 21 of 20 off Earth. Rivals: Prospectors 50% (Fund 2726/6500, 16/20 off Earth), Archivists 29% (Archive 190/250, no Upload), Arkwrights 6%: AGAIN ZERO Colonists beyond Earth orbit. They shipped every Pioneer to Antarctica by sea (counts for nothing) and filled three Antarctic Colonies.

### Game 2 verdict
- Much better game than game 1. The Custodian win is a real tug of war with the table's emissions. The Scrubber is the only lever that matters (3 ppm for 33 M; a Leapfrog is about 0.4 ppm for 50 Ducats on China), so the play is: Scrubbers to every Region's population cap, fed by solar stations and a Production-Moved Moon Generator.
- The Scrubber's cap by population (1 per 200M) makes India and China the Custodian's natural homes. Taking smaller Regions just raises the world's emissions.
- The AI Arkwrights are broken: two games, zero Colonists off Earth by T23/T29. They chase Antarctica.

## Game 3: seed 1303, Prospectors (start China)

Pre-read: the Venture Capital Fund pays 1% of its balance per "payer" a turn, compounding. A payer is each Region with an Investment Bank and each Body with an Exchange (one per Body). So k payers is k% a turn. With ~15 payers and 40-80 a turn banked, 6500 falls in about 20 turns. The whole Faction is "maximise payers early". EXCITED: a clear exponential engine.
T1: Deep Mining (Extraction chain). Investment Bank in China (23.4 M after Cheap Industry), Shipyard on Tiangong, Venture share 30%.
T3: Japan taken. T4-T5: Colony Ship to the Moon with 6; WON THE MOON RACE again (3 for 3; no AI ever beat me to it).
  - FRUSTRATED: unloading 6 into a new Colony founds it with 4; the other 2 sit aboard until a Habitat is built. Founding fills the Core's room and no more, but the order accepted 6 without a word.
T6: Exchanges on Tiangong and on the Moon (one per Body, each a payer). Ducat income 19 > 41.
T6: India taken. T9: Vietnam. T10: Opening Objective (3 Banks for 3 turns): +50 to the Fund.
T9: REFUSED the Custodians' non-aggression offer, to keep their US open as a target. T11: they marched the 1st American Army into my Mexico and occupied it; it was theirs by T13. EXCITED: the AI punished a refusal with a real war on the neighbour Region. Good.
T10-T11: emissions hit 77 ppm and the collapse projection fell to TURN 25, "Last turn to act: 21". My seat alone 28.8 ppm (Blame 47%). A Region's people emit "times its controller's Emissions multiplier", so every Region a Prospector takes gets dirtier. Thematic and fair. I replaced three Power Plants with Wind Farms and built a Nature Reserve; Clean Manufacturing landed; emissions fell to 46.7 and the collapse moved to T35. The climate finally forced a decision on me. EXCITED.
T12-T13: Venus station Ishtar founded by Colony Ship (no Materials); Exchange there.
T13: UK taken. T14: China at Industry Level 5: Second Objective, +100 to the Fund.
T15: Energy -22 a turn with 49 in stock. Bought 30 Energy and an instant Solar Array at Venus with Ducats (S yield about 2x Earth): +42 Energy a turn. Later Energy piled up to 560 with no use. Energy cannot be sold. BORED.
T16-T17: four Nature Reserves (+4 ppm sink). Indonesia taken. Moon to 18.
T18: LOST: the Extraction Charter is the gate, and with 4-6 Research a turn I would never lead a Tech to get it on a shortlist. Fixed with an Earth L4 station plus two Observatories bought with Ducats (R 2.0 at a far orbit). Thereafter I led Orbital Refuelling, Coastal Engineering, Rail Gun Freight and the Charter.
T20: the sea took my Investment Banks in India and Vietnam: -2 payers. No free inland slot to rebuild. Industry Level raise (+1 inland slot, half price for Prospectors) and rebuilt. Good interlock.
T21: Turkey. T23: Egypt (beat a Custodian contest, 43 vs 30). Extraction Charter on the shortlist; picked it. Venture share to 80%.
T24: Mars colony at Olympus Mons; Exchange and Habitat bought with Ducats the turn after. Argentina taken.
T26: THE PACK: the Custodians spent 50 on China (40 of 47, "China could fall to Custodians next turn"). Bought 12 Influence, put 47 on China and built a Constabulary (+5 margin). They kept spending 30 a turn and got to 112 of 169. EXCITED: the warning line works and the defence is a real Ducat-vs-Ducat duel.
**T29: PROSPECTORS WIN (at the head of turn 29).** Fund 6934/6500 (+1084 last turn, 877 of it interest), 42 of 20 off Earth. Temperature +2.65 C (collapse at 3.0). Rivals: Custodians 52%, Archivists 23%, Arkwrights 13% (5 Colonists off Earth, on Mars; still pouring Pioneers into Antarctica).

### Game 3 verdict
- Fund curve: T10 178, T14 483, T18 864, T21 1691, T23 2367, T25 3379, T27 4906, T28 5850, T29 6934. Classic exponential: half the bar came in the last 3 turns.
- The interest rate is what you optimise: about 15 payers by T25. Everything else (Moon, Venus, Mars, L4) was bought mainly as an Exchange slot or for the gate's Research.
- Same "winning on turn 29" as the Custodians, but the Prospectors felt more scripted. Expansion feeds itself (more Regions, more Banks, more Ducats, more Influence to buy), and the only brake is the climate, which I could hold off with Wind Farms and Nature Reserves.

## Game 4: seed 1304, Arkwrights (start China)

T1: Expanded Habitats (Off-world chain to Generation Ships). Station over Earth (Orbital Reef) at 33 M, founded at once with the 4 Pioneers I start with. Japan Influence.
T2: Shipyard on the Reef (28.9 M at the station rate). Accepted the AI bloc's non-aggression again.
T4: Colony Ship (7 aboard with Coach Class and Expanded Habitats). Japan taken.
T5-T6: Moon, WON THE MOON RACE (4 for 4). The Spaceport pays 1 Influence per Pioneer loaded: "+7 Influence" per full ship. EXCITED: a lovely Faction-flavoured feedback loop; my Allotment ran 10 > 18 > 35.
T6: Opening Objective (a Colony on the Moon): free Launch Site in Japan.
T7: Mars window: second ship with 7 to Mars. India taken.
T8-T10: Fuel is THE Arkwright constraint (income +0.8 at one point). Wind Farm for the Fuel-burning Power Plant; Refineries in India, Indonesia, on the Moon and on Mars. The Moon Refinery is a depot for the ferry. Good interlock.
T12: arrived at Mars the SAME turn as the Custodians. Both unloaded; they won the First to Mars lottery. EXCITED, fair. My colony still founded at Valles Marineris.
T13: Second Objective (beyond Earth and the Moon): a free Colonist.
T14: Indonesia. Energy and Fuel managed; the Moon at 22.
T15: EXPLOIT-ish: took 4 Colonists FROM my Mars colony and hopped them to Phobos (1 turn, 3.8 Fuel, no window). First to Phobos (+20 Influence) and a third Body with 4, leaving Mars at exactly 4. Repro: any 8-person Mars colony plus an empty Colony Ship at Mars.
T16: Moon to Settlement. Then 6 Modules in one turn: three by Materials (12-16 each with Build Where You Dig), two by Ducats (Trade Post 31, Barracks 24.8).
T17: EXPLOIT: the Moon's Trade Post paid +60 DUCATS A TURN (it pays per Colonist of mine at its Body: 23 then). It is 3x my whole Earth economy, from one 20 M Module. Repro: Trade Post at any colony of 20+.
  - FRUSTRATED (mine): forgot my Influence line one turn and lost Kazakhstan 37/38 to the Custodians' 30-a-turn contest. The Contest AI works.
T18: Moon to Colony tier (55 M). Venus window: 7 to Venus. Mexico taken.
T19: Colonial Capital (77 M) the turn after; Generation Ships on my shortlist (I led Regolith Printing). Arcology chosen.
T20: Mars window: 9 more to Mars. Generation Ships complete. The Prospectors built their first warship (Frigate) and parked it at the Moon on Attack; bought a Battery for the Moon colony with Ducats.
**T21: ARKWRIGHTS WIN, turn 21 of 50.** 46/44 beyond Earth orbit, 4 of 4 Bodies (Moon 35, Mars 4, Phobos 4, Venus 4), Capital at Oceanus Procellarum. Rivals: Custodians 34%, Prospectors 12% (Fund 644), Archivists 11%.

### Game 4 verdict
- The "weakest" Faction was my FASTEST win. The Arkwrights are not weak; the AI plays them badly (zero off-Earth Colonists in games 1-2, Antarctica-dumping in all four).
- Fun, busy turns: ferry logistics, Fuel, windows, Body counting and the Capital chain all interlock. The best-feeling Faction of the four.
- Too easy: 44 Colonists sounds big, but ships carry 7-9, the Moon is a one-turn hop, and Habitats hold 10 each. The 4-Body rule was the only real constraint, and a Phobos hop from Mars beats it.

## Final report

### 1. Per-Faction summary

| Game | Faction | Outcome | Victory | Rivals at the end |
|---|---|---|---|---|
| 1 (1301) | Archivists | **Won T23** | 100% (Archive 250/250, 16/16 uploaded) | Cust 38%, Pros 31%, Ark 6% |
| 2 (1302) | Custodians | **Won T29** | 100% (run 4/4, 21/20 off Earth) | Pros 50%, Arch 29%, Ark 6% |
| 3 (1303) | Prospectors | **Won T29** | 100% (Fund 6934/6500, 42/20 off Earth) | Cust 52%, Arch 23%, Ark 13% |
| 4 (1304) | Arkwrights | **Won T21** | 100% (46/44, 4/4 Bodies, Capital) | Cust 34%, Pros 12%, Arch 11% |

- **Archivists**: a clean engine. Lead every early Tech for +5 Influence, put an Observatory over Earth, take a cheap neighbour every 2-3 turns, reach the Moon by T8, then fund at 100% from T10. Fund full T16, Colony tier T18, Archive T19, then 4 uploads a turn. The gate Tech fell on T6 as a side effect, and the designed tension (lead or fund) barely bit. Solved on the first play.
- **Custodians**: the best game. Stabilization is a fight against the whole table's emissions, which climbed from 50 to 62 ppm while I scrubbed. Scrubbers are capped by population, so China and India are the right homes. Expansion raises world emissions: a real trade-off, but hidden. Two turns from the end, the pack nearly took China with my six Scrubbers in it. I won with one turn to spare.
- **Prospectors**: an exponential. The Fund pays 1% per Bank Region and per Exchange Body each turn, so the game is "maximise payers", and half of the 6500 came in the last three turns. The climate was the only brake: collapse was projected for T25 on turn 11. Wind Farms, Nature Reserves and Clean Manufacturing bought it back to T35-47. Satisfying, but scripted once seen.
- **Arkwrights**: the most fun and the fastest. Ferries, Fuel, windows, Spaceport Influence, Body counting and the Colonial Capital all interlock: 44 Colonists on 4 Bodies by T21. The AI makes this Faction look weak; it is not.

### 2. Lost / bored / excited (strongest moments)
- **EXCITED**:
  - G2 T22-T29: Custodian Scrubbers racing an AI table whose emissions kept rising, then the Arkwrights' "China could fall to Arkwrights next turn" with six Scrubbers inside.
  - G3 T11: two turns after I refused the Custodians' Accord, they invaded my Mexico.
  - G3 T10-T12: the climate line dropped to "collapses on turn 25" and made me rebuild my Energy around Wind Farms.
  - G4 T12: both ships reached Mars the same turn, and First to Mars went to the lottery. I lost it.
  - The Moon race Moment, every game.
- **LOST**:
  - The 224 KB glossary, mostly version history.
  - No Faction multipliers in the text tool.
  - "Next pick: Arkwrights." printed while the game demands that I pick.
  - An Earth L4 founding needs 4 aboard, and nothing warns before the 16.3-Fuel change of orbit.
  - A far orbit is reached by `change-orbit`, not `transit` ("already there"), yet the help says change-orbit costs 1 Fuel; to L4 it charged 16.3.
  - No emissions by source in the CLI, so I could not see why the world jumped +9 ppm (it was a Region I had taken).
- **BORED**:
  - Mid-game Influence as the Custodians: once more Regions mean more emissions, nothing is worth buying.
  - Energy piling up to 560 with no use (G3).
  - The last turns of G1: upload 4, end turn.

### 3. Frustrations (ranked)
1. **A Refuel drains the Stockpile's Fuel and blacks out your Power Plants at the next Income.** It hit me five times in three games (G1 T4 and T12, G2 T12-13 and T17, G4 forced pre-buying). The CLI gives no Shortfall forecast. Refuel should hold back the plants' upkeep, or warn.
2. **Order lists are checked top-down with no look-ahead.** `buy fuel` after `build ship` refuses the ship. The Module slot check ignores Colonists unloading in the same list.
3. **Rival offers (Accords, Trades) and some Choice Cards appear only when `turn` refuses**, never in `show`.
4. **A new Colony takes 4 of the 6-9 unloaded and the rest stay aboard** (the Core's room is 4), with no warning at the order. It happened in every game.
5. **One Labour Dispute card switched off all six Scrubbers in India**, and the Sink fell from 37 to 25 in one turn (G2 T20). A Stabilization run is too exposed to a single card.
6. **The raw string key "[exodus_call_order]" shows in the Rival report** (G2 T15, T21).
7. "(the Custodians's, cancellable)" on a rival's build at a place I do not direct (G1 T3): wrong possessive, and I cannot cancel it.

### 4. Exploits
1. **Ducat buy-outs are underpriced** (strong; I used it in every game). `build module-ducats` and `facility-ducats` cost 2 Ducats per Material of the discounted price, need no Widgets, and land at the next Resolution. Materials cost 3 Ducats at market and still need the Widgets. Off Earth, where a Core makes 4 Widgets a turn, this is THE build button. Repro: any colony, `build module-ducats <colony> habitat`. Fix: price it at market x2 (6 per Material), or keep a Widget floor.
2. **A Trade Post at a populous colony** (strong). G4 T17: the Moon's Trade Post paid +60 Ducats a turn with 23 of my Colonists on the Body, three times my Earth economy. Repro: a Trade Post at any colony of 20+. Fix: cap or taper the per-Colonist term.
3. **An Observatory over Earth or at L4** (strong). The R 2.00 yield makes 7-8 Research for about 31 Materials; a Research Lab makes 1.1-1.8. Every Faction should rush one, and the Lab is a trap. Fix: lower Earth orbit's R yield, or raise the Lab.
4. **Production Moved with a Moon Generator** (medium, Custodians). Mothball one Earth Power Plant and the Moon Generator runs at x2.08, x3 again under a Helium-3 card: +70 Energy from one Module (G2 T20).
5. **Splitting a colony to claim a Body** (medium, Arkwrights). Load 4 off an 8-person Mars colony and hop to Phobos (1 turn, 4 Fuel, no window). That pays +20 Influence for First to Phobos and adds a Body to Diaspora, so the 4-Body rule costs almost nothing.
6. **The Moon race is free for the human** (medium). I won it 4 of 4, landing T5-T8, with a straight Shipyard > Colony Ship > Moon opening.
7. **Fund compounding** (by design, but worth stating). With about 15 payers the Fund grows 11% a turn, and the 6500 bar is a T29 win without strain.

### 5. Suggestions

**a. More interesting and compelling**
1. **Make the AI compete for the same win paths** (see d). Every game was decided by about T20, and no rival passed 52%. Solves: no tension in the second half (all games).
2. **Show the hidden trade-offs.** On the Region: "Taking it starts its idle Facilities: +N ppm". On the people line: the controller's Emissions multiplier. Solves G2 T11 and G3 T10, where the best decision in the game was invisible.
3. **Give the Custodians and Archivists a use for mid-game Influence**: buy Leapfrogs with Influence, or spend Influence on a neutral Region to idle its Facilities. Solves G2 T17 BORED.
4. **Make Energy convertible**: sell it at half, or an Energy-for-Research Module. Solves G3, where 560 Energy sat unused.

**b. Balance: bolster the Arkwrights' and Archivists' wins**
In my hands both won FASTEST (T21, T23). Their low win rates are an **AI problem**, not a Faction-strength problem. Do NOT buff the Factions; fix the AI first:
1. **AI Arkwrights: stop sending Pioneers to Antarctica.** It counts for nothing toward Diaspora; weigh Moon, Mars and Venus landings above it. They had 0 Colonists beyond Earth orbit at T23 (G1) and T29 (G2), and 5 in G3.
2. **AI Arkwrights: build a Refinery and a Trade Post at their first colony.** That is where their Fuel and their Ducats come from.
3. **AI Archivists: build an Observatory over Earth before a Lab, and fund the Archive only once The Upload's chain stands.** In G2 they funded from T3 at about 4 a turn and never led a Tech: 190/250 by T29, The Upload never researched.
4. Only if the fixed AIs are still weak:
   - Archivists: the Archive's place must reach Colony tier (18), so let Uploads draw from any colony of theirs on that Body.
   - Arkwrights: count Earth L4/L5 as a Body for the four.

**c. UI fixes and text reductions**
1. Forecast a Shortfall in the CLI's "After them:" line: "After them: 0.2F (2.4 short for upkeep: 3 plants at a quarter)".
2. Print pending offers in `show`: "OFFERS: Prospectors: 20 Energy for 20 Fuel (trade accept|refuse prospectors)".
3. Warn at the order when a founding leaves people aboard: "founds with 4; 3 stay aboard until a Habitat stands".
4. Text cuts:
   - "Next pick: Archivists." while a pick is owed > drop it, or "Leading the next Tech: Archivists".
   - "Overtime at the Yards is asking the Arkwrights a question, and it has not been answered. Answer it. The turn cannot end until you do." > "Answer the card: Overtime at the Yards."
   - "STILL OWED: Several things before the turn can end:" > "Owed:".
   - "Prospectors lifted 2 Pioneers from Canada to Tiangong over Earth, recruited 2 Pioneers in Canada, spent 10 Influence on Russia, bought 5 Influence, spent 5 Influence on The United Kingdom and ..." > group it: "Prospectors: +4 Pioneers; 15 Influence (Russia 10, UK 5); Exchange at Tiangong."
   - "REFUSED `build module mareimbrium mine`: full until more Colonists live here" > "REFUSED: no free slot (4 of 4; 2 land this turn)".
5. **GLOSSARY.md: lead each entry with the current rule** and move every "since version X ... until Y" to a history appendix. The Archive entry, for one, carries seven version clauses before the rule is clear.
6. Possible hover mismatch in mouseovers.md: "Custodians. Hovering the Sink bar or the "{run}/{bar}" figure beside it:" is followed by an Allotment text ("Influence each turn: ..."). Check that this is the right string.

**d. AI**
1. **The pack wakes too late and too weakly.** At 72-90% it spent 5-50 Influence a turn on a Home Region priced 85-169. Wake it from 50%, pool its Influence on one Region, and combine Agitate with war.
2. **Contest neutral climbs earlier.** In G1, ten Regions fell to me uncontested. In G4 the Custodians did contest Kazakhstan and win it, so the rule fires sometimes.
3. **Use warships.** In four games the only AI warship was one Prospector Frigate (G4 T20), parked over my colony and never fighting. An AI should blockade the leader's ferry colony (the Moon) and choke Diaspora or Archive throughput.
4. **The Arkwrights' Antarctica habit** (see b1).
5. **Weigh the Moon race.** No AI ever beat my T5-T8 landing, though they had Shipyards by T3.

**e. Colonization**
1. **Raise the first colony's build rate.** A Core's 4 Widgets a turn makes every off-Earth build crawl unless you pay Ducats (exploit 1). Give the Core 6 Widgets, or make the first Factory free.
2. **Make the Bodies differ.** In all four games the Moon, a one-turn hop, was the only place that mattered. Give Mars and Phobos a reason: Fissionables fed only Reactors, which I never needed. For example, Mars Observatories at x2 Research, or Phobos as a cheap depot.
3. **Make growth matter.** 2% a turn is invisible: 1 Colonist every 2-3 turns at a colony of 20. Make the Arcology, or a hospital-type Module, count.
4. Ferry routes looked good on paper, but with a one-turn Moon hop I never needed one. Long routes (Mars) would need them if the windows bit harder.

**f. Conflict, war and spaceships**
1. **Give war a reason before T20.** Orbital Control and Blockade are rich rules, but no AI used them. Seed one AI a game with an aggressive personality that blockades the first ferry colony it sees.
2. **Spaceships, give warships targets that move a Victory bar:**
   - a Frigate can ambush a Colony Ship mid-transit at a window;
   - a Battleship can bombard an Archive or Capital under construction, for a one-turn delay rather than destruction;
   - a Carrier can seize an Exchange colony, a payer swing against the Prospectors.
3. **Batteries were the only defence I ever bought**, and only after I saw a Frigate. Make the Threats line louder in the CLI's colony row; today it says only "(threatened)".

**g. My choice: AI DEPTH (each AI reads its rivals' Victory bars)**
Every other system I touched interlocks well: climate against expansion, the sea against coastal Facilities, Fuel against ferries, payers against Influence. But the AIs play their own scripts and never read mine. They let me take ten Regions, win every Moon race, and ignore four warships' worth of reasons to hit my one ferry colony. A grand-strategy player judges a game by whether the AI forces them to adapt; in four games I changed plan because of an AI only twice (Mexico invaded, China nearly bought). The fix: each AI computes every rival's turns-to-win, and when one is under 10 it switches to denial at that Faction's chokepoint:
- the Archivists' Archive colony;
- the Custodians' Scrubber Regions (Agitate, Influence);
- the Prospectors' Bank Regions and Exchange colonies;
- the Arkwrights' ferry colony and Mars landings.

### 6. Rating: 6/10
As a hardcore 4X player: the bones are better than most indie strategy games I have tried. Climate, economy and colonization genuinely interlock, each Faction plays differently, and the Custodian game made me sweat. The sea eating my coastal Banks, and Prospector expansion heating the world, are the second-order effects EU4 and Vic3 players live for. But I won all four Factions on the first try, between turn 21 and turn 29 of 50, against AIs that never seriously contested me. Three of the four wins ran on an engine I could see by turn 5: Observatories, Fund payers, Moon ferries. The Ducat buy-out and the Trade Post are loose. And the text interface hides the two numbers I most needed: emissions by source, and the Shortfall forecast. I would pick it up again for the Custodian puzzle and the Arkwright logistics. I would tell a friend "promising, try the Custodians", and I would put it down once the AI stops being a speed bump. With an AI that plays for denial, this is a 7.5-8.
