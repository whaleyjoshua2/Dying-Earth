# Playtest notes - tester-3 (hardcore 4X persona), version 0.1.0.0

## Game 1 - Custodians, seed 301, start The United States
- T1 setup: `help` does NOT list region ids though the task/README say it does; I found them by feeding a bad `--start`. Board prints both "NorthAmerica" (id) and "The United States" (name); the error list prints the *names*, but the order wants the *id* (`northamerica`). `united` silently matched the UK. Confusing.
- T1: picked US over China/Japan/India: GDP 23 -> 13.8 D/turn and 19 Influence vs China 9.6D/14. Start choice matters a lot; no in-game comparison of starts.
- T1 QoL: NO facility/module costs or effects anywhere in help or show. I probed costs by submitting orders and reading "ok ... costs 20 Materials and 4 Widgets" (a refused line stops the turn - a nice accidental dry-run). Effects of Bank/Embassy/Constabulary/School/Stadium/Academy/Nature Reserve are a total mystery. For a numbers player this is the #1 problem.
- T1: tech `efficient` is ambiguous with `efficienttransit`, a tech not on the offered list - hidden tree?
- T2: all three AIs struck non-aggression with each other on turn 2, then offered me the same next turn. Accepted all. Felt like the table ganging up for a moment, then fine.
- T3-T7: Influence land-grab is THE early game. I got Japan (T3), Mexico (T5), India (T8). Lost the UK to a 40-Influence Prospector snipe after I'd sunk 30 into it (T7) - frustrating because `show` only prints MY Standing in a neutral region, never rivals'. QoL: show the leading rival's Standing (or at least a "contested" flag).
- T5: AI Archivists built a School in Turkey, mothballed it the next turn and decommissioned it the turn after. Wasted build - AI looks dumb there.
- T5: Leapfrog (Custodian order) costs 50D and is never explained in help ("leapfrog <state> - the Custodians only", nothing else). It cuts a Region's per-capita emissions (US to 0.13/100M). I spent 50D blind to find out. Engaged once I saw the result.
- T4: Scrubber = +3.5 Natural Sink (8 -> 11.5). Custodian Stabilization needs WORLD emissions (50-60) under the sink (~14). Feels near-impossible single-handed; the win is about policing everyone. Interesting, but daunting.
- Exploit-ish: Custodians/Prospectors' "18 off Earth" can be met by stacking Habitats on the starting ISS and using `lift` every turn - no ship, no transit, no risk. ISS 10 colonists by T8.
- T11: Arkwrights offered me a Trade and NOTHING tells me its terms - not show, not the report, not the STILL OWED line. I accepted blind (it was 20 Fuel for 15 Materials). Same for Accord offers: terms never shown. Big QoL hole.
- T8: Accords I accepted on T2 never appeared as standing; offers came again T8 and those stuck. Then they lapsed after 5 turns silently and came back at T14 - fine, but I had no warning in the report that mine lapsed.
- T11: max 2 Scrubbers per Region - only learned by refusal.
- T12: Planetary Stewardship (my victory gate) was offered on the shortlist by T12 and done by T14. Gate is easy; the Stabilization number is the real wall.
- T13: Prospectors sniped Egypt the turn I put 26 in (2nd time). They `buy-influence` 40-65 points a turn off Investment Bank income. They are the runaway AI: 8 regions by T13, 36 of the world's 66 emissions.
- T13 BUG?: `agitate china` charged 15D + 5 Influence and the report says "China: Unrest from 0 to 0 (agitation by Custodians)". Paid for nothing. Also agitate was ACCEPTED while I had a non-aggression Accord with the Prospectors (help says that forbids spending Influence on their places).
- T15 EXCITING: the Prospector Accord lapsed (they'd gone Cold after my agitate). I bought 40 Influence and dumped 68 into China (57 needed) - took their CAPITAL with 4 Investment Banks. Same turn, they took my home, the US (my Standing there had decayed 50 -> 35 untouched). A capital swap! Best moment so far. Lesson: home Standing decays every turn and nothing warns you.
- Decommission costs +2 Unrest (shown in the order echo, nice) and refunds half Materials.
- T16-17 CONFUSING: when I took China its two Armies (standing in Kazakhstan on Attack) became mine and promptly attacked the Prospectors' army there - a war I never ordered. Then I was Hostile with them and they invaded Mexico and the US militarily. Inherited stances should reset to Hold, or the report should warn.
- T18/T20 BUG?: `accord accept arkwrights` echoed "answered: you accept" but no Accord stood afterwards and the Arkwrights offered again next turn. Third try (T22) stuck. No message ever said why the first two failed.
- T19 EXCITING: sold 100 Fuel, bought 100 Influence, retook China (58) AND the US (62) in one turn. Influence-buying is the strongest lever in the game; Ducats -> Influence -> whole nations.
- T21: Sea level at +1.8C ate ~30 coastal facilities across the world in one report. Dramatic and well telegraphed by the reports. Liked it.
- T23 FRUSTRATING: put 50 Influence into Brazil which needed 228 (it had needed 37 two turns earlier; the "challenge margin" after a capture is huge). Order echo just says "ok costs 50 Influence". QoL: echo "your Standing would be X of Y needed".
- T25: Prospectors took China back again at 119 Standing. The Influence tug of war is endless ping-pong - fun for two swaps, tiring by the fourth.
- Custodian end state T25: Stabilization 41 ppm over the sink (world 88 vs sink 46.5) and widening, because the Prospector AI alone emits 57. 22/18 colonists off Earth (ISS stack). Gate done T14. No realistic path to my win - the Custodian win is hostage to the AI's emissions and I have no tool that actually cuts a rival's emissions (agitate did nothing, smear only touches the Blame ledger). Prospectors at 2952/3500 Fund, Arkwrights 39/42.

### Game 1 summary (Custodians)
Strong opening via Influence land-grab (8 regions by T15), Leapfrogs and Scrubbers felt good, the Influence capital swaps were the high point. But my victory metric moved AWAY from me all game; by T20 I knew I could not win and was only playing spoiler. Prospector AI is a competent, ruthless snowball.

## Game 2 - Prospectors, seed 302, start The United States
- T1: report prints the "[Note] Your rivals..." line twice (cosmetic).
- T2 ENGAGED: Investment Bank costs 21.3M and in the US pays +11.5 Ducats a turn. Ducats 16.6 -> 39.6 a turn after two banks.
- T4 BUG: the shortlist says "Large Language Models" but `tech largelanguagemodels` is refused; the id is `publicscience`, which appears nowhere on screen. Had to guess from the refusal's id list.
- T4 EXPLOIT-ish: Strip Permit is free and doubles a Region's Facilities for 3 turns: US Ducat income 39 -> 122 the next turn. The cost is a permanent +0.x baseline emission (which I can greenwash off my Blame for 1 Influence + 1 Ducat per 2 ppm). Strip-permit-the-banking-capital is a no-brainer.
- T5: lost the EU to the Archivists and missed the UK by 2 points - the "needed" figure moves between my order and the Resolution (my Blame multiplier x1.20 and my Standing decay of 1/turn). Order echo should warn "may fall short at Resolution".
- T9 HIDDEN NUMBER: Investment Bank yield scales with the Region's GDP: +11.5D in the US, +1.5D in India/UK, +0.5D in Vietnam (and -2E upkeep). Nothing tells you that before you build. I built 6 near-worthless banks.
- T9 DEGENERATE: buy-influence is a flat 2 Ducats a point, unlimited. Bank income -> buy 50 Influence -> take a GDP-16 capital (China from the Custodians at T9, 66 points). By T10 I hold 9 Regions. The Prospector money loop snowballs exactly like the AI Prospectors did in game 1.
- T10 AI: the Custodian AI mothballed its own Power Plant, Refinery, Mine and both Scrubbers in China before I took it. Looks like an AI energy-panic loop gutting its own economy.
- T11: Tiangong refused a module: "full: upgrade to a Settlement for more". There is no upgrade order in `help` - I never found how to upgrade. Confusing.
- T12: Carbon Levy card says "+0.5 Unrest in every Region you hold"; the report then showed Unrest FALLING (0.5 -> 0) in Mexico and Japan "(the Carbon Levy card)". Card text and effect disagree.
- T13 KEY INSIGHT / EXPLOIT: Investment Banks also pay "interest banked" into the Venture Fund in proportion to the fund (8 banks -> +28.8D/turn at fund ~750). So even 0.6D-yield banks in poor Regions are worth spamming: take every cheap neutral with buy-influence, fill every slot with banks. Fund went 523 (T12) -> 1001 (T14) -> 2094 (T17) -> 3086 (T19) -> 3669 (T20). Exponential.
- T14: mothballing my Refineries + 2 Power Plants cut my emissions 76 -> 60 for +1 Unrest each. I had 470 Energy banked and 79 Fuel a turn I couldn't use. Fine trade.
- T15: the Extraction Charter showed on the shortlist at T15 (after I'd led picks Commodity Finance, Rail Gun); done T18. As Prospectors I held the Research Lead almost every turn and steered the tree straight to my gate.
- T16: heat Unrest +2 in every region every turn from ~+1.9C; Relief at 10D a point is cheap enough to ignore it.
- T20 WIN: "Game over, turn 20: Prospectors win" - printed as an ordinary [Note] line in the middle of the "On Earth" section. A win deserves a HEADLINE / banner. The status line on the turn I passed 3500 still said "The game is still on" with "score 1.00 MET" - win is checked the turn after, which is fine but could say "victory at next Resolution".
- AI read: Custodians and Archivists smeared me +20 ppm/turn every turn from T8 on; I greenwashed 40/turn for 20 Influence + 20 Ducats. A wash. The AIs never tried to take a Region back from me by Influence or force in this game. Too passive against a runaway.

### Game 2 summary (Prospectors)
Won on turn 20 of 50, never threatened. Investment Banks + Strip Permits + unlimited 2D Influence purchase = I took 15 Regions, including the Custodians' capital at T9, and the fund compounds via bank interest. It felt great for 10 turns and then like a formality. The Prospectors are clearly overtuned (and the AI Prospectors also ran away with game 1). Climate reached +2.15C at T20 - had the game run on, I'd have helped cook the planet; nothing stopped me.

## Game 3 - Arkwrights, seed 303, start The United States
- T1: Arkwrights start with no station. `build station earth 3` costs only 20M for them and it arrives with 4 colonists - discovered by trial; nothing says so.
- T2-T6: lift + emigrants pipeline (3 Pioneers/turn, Spaceport +1 Influence per lifted colonist) is satisfying. Materials (8/turn at start) are the real choke.
- T7: one order per Ship per turn means `load` then `transit` costs two turns - I loaded on the Mars window turn and left the turn after. Fine once known; help doesn't say it.
- T9: Research Directive for the Arkwrights (Research -> Fuel) paid 0.4 Fuel for 2 Research. Useless.
- T10: A Colony Ship costs its own tank in Fuel (35, 40 after Clean Propellant). Fuel, not Materials, gates the Arkwright tempo.
- T11-16 ENGAGED: Moon (T11), Mars (T14, 5-turn window flight), Mercury (T16, 2-turn hop with 0.6 Fuel left, ship marked STRANDED). New colonies hold 4, the rest "wait aboard" until a Habitat lands - good, readable.
- T15: "a Solar Array stands only on a station"; "Generator: not on a station". Fine rules, but only discoverable by refusal.
- T16 + T18 FRUSTRATING: lost my home (US) to Custodian Influence (Standing decayed 50 -> 35), bought it back with 40 Influence, and the Custodians' inherited US army then marched into Mexico and the Custodians OCCUPIED the US by force two turns later. Same inherited-army-attacks pattern as game 1. Losing the US killed my Spaceport, Launch Site and Ducat income (25 -> 5.6).
- T20/21 BUG: Venus. Ship arrived at Ishtar orbit with 8 colonists. `unload ... into slot venus 0` -> "Move this Ship to Venus, low orbit, then found the Colony". `change-orbit 34 low` -> REFUSED "Venus has low orbit and 3 Orbital Slots, numbered 0 to 2" (contradicts itself; Venus' orbit list shows no low orbit and no ground). 8 colonists and a ship lost. The board even advertises a Venus launch window.
- T19+ MISSING FEATURE?: the README says the Arkwrights must also hold a Colonial Capital (24 people, 70M). My Moon colony reached 25 colonists; there is no order to build/upgrade a Settlement or Capital (tried `build module 30 capital`, `build capital`, `upgrade`), `show` never prints a colony tier, and the victory line doesn't list the Capital. On T27 the line reads "60/42 | 4 of 4 Bodies | score 1.00 (held: needs Generation Ships)". Either the Capital isn't enforced or the text UI can't show/do it.
- Report grammar: "at on the ground at the Moon, reached from the Moon, low orbit".
- AI: Archivists quietly founded colonies on Moon, Vesta x2, Phobos, Ceres and filled the Archive 225/225 by T25 - the strongest AI this game. Prospector AI much weaker than game 1 (fund 1201 at T25).

### Game 3 summary (Arkwrights)
The most "space game" of the four and the one I enjoyed most: windows, tanks, stranded ships, 4 bodies by T26 and 60/42 colonists. Only the Generation Ships tech (never offered while I wasn't Research Lead - I make ~5 Research/turn) held the win. Lost my capital twice to Influence/occupation without warning. Venus is a trap (bug).

## Game 4 - Archivists, seed 304, start The United States
- T1: Research Labs + Reactor + `directive 60`; Labs pay double duty (Archive fund AND Research Lead share). By T12 my Labs made 27/turn, more than the other three seats together, so I was Research Lead nearly every turn and the table researched what I wanted.
- T7: Colony Ship capacity is 6 even though `load 26 8` was accepted in my Arkwright game - Arkwrights 8, others 6; only learned by refusal.
- T10-11: Moon colony founded with 4 (2 wait aboard), `build archive 29` the next turn (37.5M, 10 Widgets), raised T14, paid in full T16 with directive 100%. Clean, legible pipeline. ENGAGED.
- T16: Axiom showed "Modules 4/3" after I shipped colonists out - the module cap is tied to population tier and can go below what's built. Odd but harmless.
- T17 (4th time across 4 games): my home Region (US) lost to rival Influence because home Standing decays ~1/turn. Bought it back with 85 Influence (66 bought). A "your capital is at risk" warning would save a lot of grief.
- T18-22: the Victory gate path is hidden: The Upload only appeared after I chose Superconductors (rung 3, Energy). README says "each behind a new 58-Research Tech of its own branch" but the game never shows the tree or prerequisites. I guessed via Orbital Data Centers first (wasted pick). QoL: a `techs` command that prints the tree.
- T22: Moon colony at 19 people: "full: upgrade to a Settlement for more" - again, no way to upgrade from the text interface.
- T23 WIN: `upload 29 18` -> "Game over, turn 23: Archivists win". Again just a [Note].
- AI: Prospector AI took my US once; Custodians offered blind Trades; nobody contested the Moon or the Archive. Arkwright AI founded Mercury and Deimos. No AI ever attacked a colony in space in any game.

### Game 4 summary (Archivists)
Won on turn 23. Labs -> Research Lead -> steer the tree to your own gate, while the same Labs fill the Archive. One Moon colony, three colony-ship round trips, done. Satisfying engine, but once the Archive filled it was a race against nothing - no rival can stop an Upload.

---------------------------------------------------------------------
# FINAL SUMMARY

## Per Faction
- **Custodians (G1, seed 301):** no win, and none in sight. 8 Regions, 9 Scrubbers, 7 Leapfrogs, gate done T14, 22/18 off-Earth by stacking ISS. But Stabilization needs WORLD emissions under the Sink, and the Prospector AI alone emitted 57 of 88. I had no tool that cuts a rival's emissions. The best moment was a capital swap with the Prospectors (T15/T19).
- **Prospectors (G2, seed 302):** won T20. Investment Banks + Strip Permits + unlimited 2-Ducat Influence = 15 Regions, and the fund compounds through bank interest. Never threatened.
- **Arkwrights (G3, seed 303):** no win by T27 (60/42 colonists, 4/4 Bodies, held only by Generation Ships, which I never got as Research Lead). Most fun of the four, with real logistics. A Venus bug stranded 8 colonists, and I lost the US twice.
- **Archivists (G4, seed 304):** won T23. Labs feed both the Archive and the Research Lead, so I steered the table straight to The Upload.

## Top 5 frustrations
1. Hidden numbers: no costs, yields or effects for any Facility or Module in help or show. I found them only by submitting orders.
2. Trade and Accord offers must be answered blind, because their terms are never printed. Accepted Accords sometimes silently fail to stand.
3. Home Standing decays every turn with no warning. I lost my capital to Influence in all four games.
4. The Tech tree and gate prerequisites are invisible. The shortlist shows 3 picks and you guess the branch.
5. Inherited Armies on Attack start wars you never ordered (G1 and G3).

## Top 5 delights
1. Influence capital swaps: buying 100 Influence and flipping China and the US in one turn.
2. Arkwright logistics: launch windows, tank math, STRANDED ships, colonists "waiting aboard".
3. The climate as a shared clock: Coral Die-off, Permafrost, sea level eating ~30 coastal facilities, heat Unrest. Dramatic and well reported.
4. A refused order line stops the turn, which works as a free dry run, and every ok line echoes its exact cost.
5. The Research Lead mechanic: the biggest researcher steers everyone's tree. It is a real strategic lever.

## Exploits found
- **Influence for cash:** `buy-influence` costs a flat 2 Ducats a point with no cap and no rising price. Ducats turn straight into Influence and Influence into whole nations: China at T9, the EU at T11, and 3 neutrals in one turn at T13.
- **Bank interest snowball (Prospectors):** every Investment Bank adds "interest banked" to the Venture Fund in proportion to its size. A bank in even a 0.6-Ducat Region is worth building, so take cheap neutrals and fill every slot with banks. The fund went 523, then 1001, 2094, 3086 and 3669 over T12-T20.
- **Strip Permit:** free, it doubles a Region for 3 turns (US Ducats 39 -> 122), and the permanent emission it leaves is cheaply greenwashed off your Blame.
- **Greenwash beats smear:** 20 Influence + 20 Ducats a turn cancelled two AIs smearing me +20 ppm a turn.
- **Off-Earth count by stacking (Custodians/Prospectors):** stack Habitats on the starting station and `lift` every turn to meet "18 off Earth" by ~T11, with no ship and no risk.
- **Research Lead monopoly (Archivists):** Labs feed the Archive AND the Lead share, so you choose the table's tree and beeline your own gate.
- **Mothball to cut emissions:** mothballing unused Refineries and Power Plants cut 16 ppm for +1 Unrest each, which Relief at 10 Ducats a point mops up.

## Bugs / inconsistencies
- Venus can't be settled. `unload` says "move to low orbit", then `change-orbit low` is refused with "Venus has low orbit and 3 Orbital Slots", yet the orbit list shows no low orbit.
- The shortlist shows "Large Language Models" but its order id is `publicscience`.
- `agitate china` in G1 cost 15 Ducats + 5 Influence for "Unrest 0 to 0". It was also accepted while a non-aggression Accord stood.
- The Carbon Levy text says +0.5 Unrest, but the report showed Unrest falling.
- Accord acceptances were echoed but did not stand (G1, T2/T18/T20).
- There is no order to raise a Settlement or Colonial Capital, and no tier display. The Arkwright victory line omits the Capital the README requires.
- Cosmetic: a duplicate "[Note] Your rivals" line; "at on the ground at the Moon"; "Modules 4/3"; help doesn't list region ids (README says it does); a win is announced only as a [Note].

## QoL suggestions
- A `costs`/`info <building>` command, and show yields before you build.
- Print the terms of every Trade or Accord offer on the STILL OWED line.
- Show rival Standing in contested Regions, and warn when a held Region's Standing nears the challenge line.
- Order echo for influence: "Standing would be X of Y needed at Resolution (after decay/Blame)".
- A `techs` command that prints the tree with prerequisites and each Faction's gate path.
- A big GAME OVER / VICTORY banner. Reset inherited Armies to Hold.
- Accept the region display name in orders (`"The United States"` -> northamerica). `united` silently matched the UK.

## Balance observations
- Prospectors are overtuned: a T20 win for me, and the AI Prospectors ran away in G1 (8 regions by T13, 57 of 88 world emissions).
- Custodians look unwinnable. Their metric depends on rivals' emissions, and they have no lever that cuts them.
- Archivists are strong (T23) because Labs double-dip.
- Arkwrights depend on another seat researching Generation Ships. Their own research is tiny, so they rarely hold the Lead.
- A climate collapse never happened in 25 turns. +2.3C at T25 in G3, at a ~0.05C/turn pace, would hit +3.0 around T38-40.
- The AI is good at Influence sniping and weak elsewhere. It mothballs its own economy (Custodian AI in G2), builds and scraps Schools, never attacks colonies in space, and never coordinates against a runaway leader beyond smearing.

## Overall enjoyment: 6/10
There is a genuinely interesting 4X here. The Research Lead tug of war, Influence capital swaps and orbital logistics are fresh, and the climate clock gives every turn weight. As a numbers player, though, I spent half my effort reverse-engineering hidden costs, yields and tech prerequisites by feeding the parser bad orders, and answered Trades blind. Two of the four Factions won by T23 on a clear dominant loop: buy Influence, then banks or labs. One Faction (Custodians) is hostage to the AI, and one (Arkwrights) is hostage to someone else's tech pick. Fix information visibility, cap or price-scale buy-influence, and give the Custodians teeth, and this climbs to an 8.
