# Playtest notes - tester-1 (common gamer persona)

## Setup
- README talks about double-clicking dying-earth.exe and a 3D map; I have play.exe text mode. Fine.
- Brief said `help` lists region ids. It does not. `new` without --start just dropped me in China. Confusing.
- First `show` is ~250 lines. Wall of text. As a casual I have no idea where to look. Flight table is a spreadsheet.
- help text has dev "Ticket #337 (version 0.09.0)" notes in it - feels like reading a bug tracker, not a manual.

## Game 1 - Custodians (seed 101, China)
- T1: BUG/confusing: Tech list shows "Large Language Models" but `tech largelanguagemodels` refused; the refusal list doesn't contain it at all. Its id is `publicscience`. Pure guesswork. Facility "lab" refused, must be "researchlab" (fine, error lists options - that's good).
- T1-T3: Every Tech finished in ONE turn. Felt great at first (instant gratification) but then "what does research even cost?"
- T2: three rivals all offered Accords at once and the turn refused to end until I answered. Accepted all (non-aggression, free = why not).
- T3: Energy went negative last turn (-3E) after the Lab + Habitat upkeep. Nothing warned me at order time.
- T4: EXCITED: Japan flipped to me by Influence after 3 turns of 14. Clear progress bar ("Standing 26, 30 needed") = good.
- CO2 went 452 -> 564 ppm in 3 turns. Custodian win needs emissions under an 8 ppm sink while table emits ~45. Feels impossible from turn 1.
- T6: one refused line (short 1 Material) blocks the WHOLE turn. Sell/buy order matters inside a turn. Annoying but at least it tells me and I can resubmit.
- T7: Recruited Pioneers in Japan, then found Japan has no Launch Site so I can't lift them anywhere. Wasted people; nothing warned me at `emigrants`.
- T7/T9: `build station earth 4` -> "needs 4 Colonists". Which colonists? where? ISS has 11. No idea how to satisfy it. Gave up on a 2nd station.
- T8: Rivals re-offered Accords (old ones lapsed, I never noticed). Turn blocked until answered - again.
- T9: Archivists offered me a Trade but `show` never says WHAT the trade is. Blind accept/refuse. Refused.
- T8: ENGAGED - Planetary Stewardship (my win gate) was on the shortlist by turn 8. Grabbing countries by Influence is the fun loop: Japan T4, UK T6, Mexico T8, Kazakhstan T9.
- Energy piles up (80+) with nothing to spend it on and you can't sell Energy.
- T11-T17: Found the Custodian engine: every Scrubber raises the Natural Sink. Gap "emissions over sink" went 33 -> 1.5 ppm in 6 turns just spamming Scrubbers (cap 2 per country) in every country I grabbed. Feels like THE strategy; Influence-grab neutrals + 2 Scrubbers each. Possibly degenerate.
- T13: India (1.8 billion people) fell to me for 40 Influence over 2 turns. Felt too easy.
- T14: Spent 23 Influence on Nigeria, Prospectors flipped it the same turn - my points just vanished, no refund, no warning I was racing them.
- T16: Energy crash: -24.5 E/turn because each Scrubber eats 3 E. The income breakdown is ONE line ~2000 characters long. Unreadable. Need a per-country summary.
- T17: Mothballing idle Factories costs +1 Unrest each (fine), saved ~10 E/turn.
- T17: ISS hit 19/18 colonists - half my win done by T15 with just `emigrants` + `lift` from China every other turn. Never needed a ship.
- T18: Prospectors marched an army into my Kazakhstan while we had a non-aggression Accord?! It died to my standing army. Exciting, but how was that allowed?
- T18: Archivists' Archive is 225/225 already. They're the threat.
- T19: DISASTER/FRUSTRATED: my HOME, China, flipped to the Prospectors by Influence. My Standing there quietly decayed 50 -> 32 while I spent everything abroad; all accords had lapsed on T19 (5-turn timer) with no reminder. Lost my only Launch Site. No "your capital is at risk" warning anywhere.
- T20-21: Prospectors then Archivists invaded Kazakhstan; it is now "occupied by the Archivists" while I "hold it in name". China flips Archivists(Pacified)/Prospectors(Influence) in alternating headlines - confusing.
- T21: `move-army` India -> Kazakhstan refused "not a neighbouring continent". No map so I can't know what borders what.
- T22: Mothballing Factories/Refinery: emissions dropped 6 ppm and Energy swung to +60/turn. Mothballing is a stronger lever than I expected.
- T23: EXCITED: "3.6 ppm under the Sink, run 1 of 4". Can almost taste the win.
- T23: EXPLOIT: "2 Scrubbers per Nation State" is NOT a flat cap - India took 10 (cap must scale with population?). The refusal only says "holds its 2 Scrubbers already" when you hit it; no way to see a country's cap beforehand. India with 10 Scrubbers = half my sink.
- T25: "run 4 of 4 ... score 1.00 MET" printed, but "The game is still on." Had to end one more turn to actually win.
- T26: WON (Custodians win, turn 26). The win is a `[Note]` buried mid-report; the HEADLINE that turn was "China now belongs to Prospectors". Anticlimax! A win should be the headline with fanfare.
- Same turn India flipped to Prospectors by Influence - if I hadn't won I'd have been losing everything.

### Custodians summary
Won on turn 26 without ever building a ship. Loop: Influence-grab a neutral every 2-3 turns, stack Scrubbers (India took 10), lift 2 Pioneers from China to ISS every other turn, research Planetary Stewardship (available by T8), then mothball Factories/Power Plants to dip under the Sink. Space was basically irrelevant: one station, 4 Habitats. Mid-game felt like it was slipping (lost China T19), then the climb back under the sink was genuinely tense and fun.

## Game 2 - Prospectors (seed 102)
- P-T1: Same China start again (no --start given, help lists no regions). Same Japan grab by T3. Opening already feels samey.
- P-T2: Labour Dispute card idled EVERY facility in China for a turn. Brutal, random, nothing I could do.
- P-T4..: Investment Banks are the engine and stack without limit: 4 in China (GDP 16) pay ~16 Ducats each. UK/Japan IBs pay ~3. Strip Permit doubled my output for 3 turns (once per country).
- P-T9: Fund banking 83/turn by T9. ENGAGED - watching the fund number climb is satisfying.
- P-T9: Custodians (AI) invaded Mexico the turn after I built 3 Investment Banks there. Our Accord had lapsed at T5 ("Custodians let your Accord lapse") - a one-liner I missed. They are "Cold" because I'm 57% of world Blame. Makes sense thematically, felt like a sucker punch.
- P-T12: EXPLOIT-ish: `buy-influence 40` (80 Ducats) + allotment = 57 Influence = India (1.8bn people) flipped in ONE turn. Prospectors have spare Ducats so they can just buy countries. Ducats->Influence at 2:1 with no cap feels degenerate.
- P-T12: 18 colonists off Earth done by T12 with zero ships, again (lift from China launch site to my station). The "off Earth" half of the win condition is trivial.
- P-T17: EXCITED: took the Custodians' HOME (EU, GDP 17) with 54 bought Influence in one turn. They only needed 45. Rival capitals are as cheap as neutrals. Fund at 1893/3500 by T18, +137/turn.
- P: My Blame share is 65% of the world and nothing really punishes me except costlier Influence (x1.40), which Ducats paper over.
- P: Research: as Prospectors my labs make ~5-11/turn; I am never the Research Lead, so my win gate (Extraction Charter) is in other people's hands. Feels out of my control.
- P-T19-22: China ping-pong: Custodians took it by Influence (T19), I bought it back (T20, 55 Influence), their armies then occupied it (T21) - the two armies I had JUST built in China became theirs. Then the threshold to retake jumped from 93 to 123 in one turn (unrest?). Felt arbitrary.
- P-T23: Fund hit 3550/3500 with 22 colonists off Earth... and I can't win because The Extraction Charter (my gate) has never been on a shortlist I got to pick from. FRUSTRATED. The Arkwrights/Archivists hold the Research Lead turn after turn. Converting factories to Labs now as a hail mary.
- P-T25: Fund at 4793/3500 (137% of the bar), 23/18 colonists, score "1.00 (held)". Stopped at T25 with NO winner: none of the four gate Techs had been researched by anyone. Lost the EU back to the Custodians the same turn.

### Prospectors summary
Strongest economy I've had: Investment Banks stack without limit in every country, Strip Permits double output, and spare Ducats buy Influence at 2:1 so I could buy whole countries (India, EU) in one turn. Hit the money bar by T23. But I never won because the Extraction Charter never came up on a shortlist I controlled - I'm the worst researcher at the table. The last 3 turns were just "end turn and hope", which is the definition of boring. Middle game was a messy influence/occupation ping-pong with the Custodian AI over China, Mexico, Kazakhstan and the EU.

## Game 3 - Arkwrights (seed 103)
- A-T1: Figured out what "needs 4 Colonists" meant in game 1: a new station needs 4 Pioneers WAITING in a region with a launch site. In game 1 I always lifted mine immediately so I never had 4 waiting. The refusal should say where the 4 colonists must be.
- A-T5..8: First real space play. Shipyard -> Colony Ship (costs 35 Fuel, I had 9; bought it) -> load -> fly -> unload into a Moon slot. "A ship takes one order a turn" so load+fly+land = 3 turns for a 1-turn trip. Clunky, but founding the first Moon colony (+5 Influence, -1 Unrest) felt GREAT. EXCITED.
- A-T9: `transit 25 earth 0` sent my ship to slot 0 = the Custodians' ISS, not my Orbital Reef (slot 3). Slot numbers vs colony numbers vs station names is confusing; let me say `transit 25 orbitalreef`.
- A-T10: MISSED THE MARS WINDOW. T1 said "Mars window in 6 turns", I was busy, now it says "in 11 turns (March 2033)". Nothing reminded me as it approached. A "window opens next turn!" headline would save this.
- Constant Choice Cards (Refugee Convoy, Overtime, Recruiters, Hard Winter, Grounded Fleet, Whistleblower...) - one nearly every turn, and the turn can't end until answered. By T10 it's a chore; I just type `answer take`.
- A-T14..16: BUG / SOFT-LOCK: flew a Colony Ship (6 colonists) to Venus to found a station. Venus lists only orbits 0/1/2 (no "low"). `unload ... into slot venus 0` -> "Move this Ship to Venus, low orbit, then found the Colony". `change-orbit 28 low` -> "Venus has low orbit and 3 Orbital Slots" (refused). `transit 28 venus low` -> "already there". `build station venus 0` -> "a Colony Ship in this orbit founds it". Every route refuses; four contradictory messages. 6 colonists and 5 turns wasted. FRUSTRATED. Rescued by diverting to Mercury (2 turns, 17 Fuel of an 18.9 tank).
- A-T18: DELIGHT: "ENERGY ALARM: next Income is 0.6 Energy short. These go dark, in this order: the Shipyard..." - exactly the warning I wanted in game 1. More of this please.
- A-T18-21: Founded Mercury (+20 Influence, first), Vesta (+25), Mars (+15) in four turns. Ships launched at T13 all landing around T18-21 = a real payoff moment. EXCITED. This is the best stretch of any game.
- A-T21: Generation Ships came up on MY shortlist and I picked it. Then 4/4 Bodies, 47/42 colonists by T25.
- A-T22-25: BLOCKED: the last piece is "a Colonial Capital at a Colony you hold off Earth". README says 24 people and 70 Materials. My Moon colony has 25 people and I have 224 Materials. There is NO order for it: not a module (`colonialcapital`/`upgrade` refused), `industry` only takes Regions, `help` never mentions a Capital. It didn't appear automatically either. I literally cannot finish my win. Very frustrated - the Arkwrights are unwinnable in this build as far as I can tell.
- Colony Ship 28 is labelled *** STRANDED *** at Mercury (1.8 Fuel, no station). Nice that it's flagged; no way to rescue it.

### Arkwrights summary
The most fun faction: actual spaceships, launch, travel, land, "first to settle X" bonuses. The colonist and body targets were reachable by T22-25 with 6 colony ships. Pain points: one-order-per-ship-per-turn makes everything take 3x longer, slot numbers are confusing (sent a ship to the ISS by accident), I missed the Mars window because nothing reminded me, Venus is a soft-lock (can't found a station there), and the Colonial Capital - the final win requirement - has no command I can find. Stopped at T25 with score "1.00 (held)".

## Game 4 - Archivists (seed 104)
- R-T1..T15: Archivists are the research faction: I was Research Lead nearly every turn early (+5 Influence each time). Learned from game 3 and worked out the gate pattern myself: each faction's T4 gate sits behind one rung-3 Tech (Superconductors -> The Upload here). Nothing in the game tells you this; you have to notice it.
- R-T9: You can fund the Archive before it's built (it banks). Nice, forgiving.
- R-T15: The Upload on MY shortlist at T15. Picked it. ENGAGED.
- R-T17-19: `fund-archive` at 100% after The Upload landed: 45 Research/turn into the Archive, 118 -> 225 in 3 turns. Shuttled 2 colony-ship loads to the Moon colony.
- R-T20-22: `upload 29 16`, then 1, then 1. Archivists win, turn 22. Again the win was a `[Note]` under a headline about Russia changing hands.
- R-T22: The final turn was blocked by an Auditors card I didn't see because it only shows in STILL OWED. A card on the turn you win...

### Archivists summary
Smoothest, most "on rails" game: be the research leader, pick the gate Tech yourself, one Moon colony, fund the Archive, upload. Won T22 with almost no conflict - nobody attacked me. Fun because the plan was clear and progress bars kept moving; a bit too easy, and the upload itself (deleting 18 people) had no flavour text or drama.

---

# FINAL SUMMARY

## Per Faction
- **Custodians (seed 101): WON turn 26.** Grabbed neutral countries by Influence and stacked Scrubbers (India took 10), mothballed polluters to dip under the Sink. Lost my capital China mid-game to Influence decay. Tense comeback, never needed a ship.
- **Prospectors (seed 102): no win by T25.** Fund at 4,793/3,500 and 23/18 colonists by T25, but the gate Tech (Extraction Charter) never came up for me to pick. The best economy, the most frustrating ending.
- **Arkwrights (seed 103): no win by T25.** The most fun (real spaceflight: Moon, Mercury, Vesta, Mars). Hit 50/42 colonists on 4 Bodies and researched Generation Ships, but couldn't find any order that builds the Colonial Capital. Venus station founding is a soft-lock.
- **Archivists (seed 104): WON turn 22.** Research leader from T1, picked The Upload at T15, funded the Archive at 100%, uploaded 18. Clean but a bit easy.

## Top 5 frustrations
1. **Arkwrights can't finish:** there's no order for the Colonial Capital (not a module, not `industry`, not in `help`), even with 25 people and 224 Materials.
2. **Venus soft-lock:** four refusals that contradict each other (go to low orbit / "already there" / no low orbit listed / "a Colony Ship founds it"). It wasted a ship and 6 colonists.
3. **Gate Techs depend on who leads research.** As Prospectors I hit the money bar and then just pressed end turn, never getting the chance to pick my gate.
4. **Losing your capital without warning.** Standing drops 1 a turn, and Accords run out after 5 turns with a one-line note. Twice my home flipped to a rival's Influence with no "at risk" alert.
5. **Too much friction per turn.** A Choice Card nearly every turn, Accord re-offers every 5 turns, and one refused line blocks the whole turn. Trade offers don't show their terms. The income line runs to about 2,000 characters.

## Top 5 delights
1. Founding colonies: "first to settle Mercury: +20 Influence". Ships launched on T13 landing on T18-21 was the best stretch of play.
2. Taking countries by Influence: a visible "26 of 30 needed" counter, and Japan flipped on T3-4.
3. The ENERGY ALARM that lists what goes dark next. Exactly the kind of warning I wanted everywhere.
4. Clear progress bars on the Victory lines (Fund 1893/3500, run 3 of 4) gave me goals I could chase.
5. Climate Breaks (Coral Die-off, Permafrost Thaw) gave the world some flavour and stakes.

## Exploits found
- **Scrubber cap isn't flat.** India took 10 when other countries cap at 2, and it was half my sink. The cap appears to scale with population and you can't see it until you hit it.
- **Investment Banks stack without limit,** about 16 Ducats each in China.
- **`buy-influence` turns Ducats into Influence at 2:1 with no cap.** Prospectors bought India and the EU (a rival's home) in one turn each.
- **The off-Earth colonist goal is trivial** for Custodians and Prospectors: emigrate and lift from the home Launch Site, no ships needed.
- **Mothballing is a cheap emissions lever:** +1 Unrest each.

## QoL suggestions
- Name the missing order or location in refusals (for example "needs 4 Colonists" should say where).
- Show the trade terms in `show`.
- Add an "Accord expires next turn" reminder and a "your capital is at risk" alert.
- Add "Mars window opens next turn!" headlines.
- Let ships transit by station name and allow load+transit in one turn.
- Put the win in the HEADLINE, and end the game the turn the score is MET.
- Use the displayed Tech name as its command id (Large Language Models is `publicscience`).
- Have `help` list start regions and the Capital order.
- Summarise income per country.
- Say when Pioneers are recruited somewhere without a Launch Site.

## Overall enjoyment: 6/10
Under the walls of text there's a real game. Grabbing countries, watching the climate tick toward +3, and especially flying colony ships out to new worlds hooked me more than I expected for a 4X. Winning two games felt good. But I spent too much time fighting the interface: invented command ids, contradictory refusals, turns blocked by cards I hadn't noticed, and two factions I couldn't finish because of what look like bugs or missing orders. The Arkwrights game was my favourite and also the one I couldn't win, which says a lot. Fix the blockers, cut the per-turn paperwork, and this is a 7.5-8.
