# Suggestions: Research, the Tech Tree and the Event Deck

Brainstormed by an Opus agent on 2026-09-09 after reading the spec, the glossary, `techs.toml`, `events.toml` and the playtest note. Proposals only; nothing here is decided.

Two things shaped the list. First: as written, Research is the only system in the game with **no decision in it at all** — Labs produce, points flow automatically into one Tech, and the only choice is the Lead's pick. Second: all twelve Techs are multipliers, and all twenty-eight Event cards are things that happen *to* you. Both are addressed below, best first.

---

## 1. The Consequence Deck

A second, small deck of about twelve cards, never shuffled and never drawn at random: each names a board condition and fires once, in Resolution (h), the turn its condition is first met. Every condition is something a Faction did — the first Colony founded ("Ticker-tape: +5 Standing for its founder in every state it controls"); the first Battle fought inside a Nation State ("The war comes home: every state with a Standing Army above strength 2 loses 3 Standing for *both* Factions"); each Sea Level threshold crossing ("The coasts move: an exposed state's population −8% and its Influence value permanently −1"); the Temperature first standing at +2.0; the first Occupation to reach Pacified; the first Space Station over a Body other than Earth. They cost nothing to trigger and cost exactly what the card says. The Report names the condition and who met it.

**Why:** the biggest swings in the game stop being a die roll and become the visible, dated price of the player's own choices — which is what "a game about consequences" should mean mechanically.
**Size:** Medium.
**Risk:** needs the same ruling Event cards have — a Consequence card's Emissions do not count against a Stabilization run — or the Custodians lose runs to their own success. Several cards hit both Factions for one Faction's act; intended, but the Report must say whose act it was or it reads as arbitrary.

## 2. Cards that ask a question

The Event popup already stops the turn dead after End Turn is pressed; give six to eight cards two named options, chosen in that popup and applied in Resolution (h). "Refugee Convoy: take 0.4 population into your most populous controlled state and +2.0 Emissions next turn, or refuse and lose 5 Standing in every state you control." "Grounded Fleet: hold every Ship in orbit this turn, or launch anyway and every Ship takes 1 damage." "Cheap Ore Offer: 30 Materials now for 20 Ducats, or decline and the Trading window's Materials price falls to 1 for two turns." Both Factions are asked the same question in the same turn; a Faction with no legal option takes the default printed on the card, and the AI chooses through the existing scored-action machinery of 16.2, weighted toward the part of its Victory Condition it is furthest behind on.

**Why:** it converts the single most random moment of the turn into the turn's sharpest decision, which is exactly what an event deck does in a good board game.
**Size:** Medium.
**Risk:** version 0.03 (ticket #32) removed faction-targeting cards on principle. Asking *both* Factions the same question keeps that rule intact and is better theme anyway, but the AI must answer visibly in the Report or the player cannot read what the world just did.

## 3. A fourth rung: Techs that unlock instead of multiply

Add one rung-4 Tech per branch, cost 60, each changing what can exist rather than what a number is. Propulsion **Nuclear Thermal Rockets**: every transit time −1 turn, minimum 1, which puts Mars three turns out and the Martian moons four. Off-world Living **Closed Ecology**: unlocks the **Farm**, a Module that holds Colonists at no Energy upkeep and makes its Colony immune to Dust Storm. Extraction **Volatile Cracking**: unlocks the **Ice Plant**, a Module buildable only on a Body with a Refinery yield under 1.0, producing Fuel at ×2 there — it turns Phobos and the Moon from dead rock into the game's fuel depots. Industry **Orbital Fabrication**: a Shipyard on a Space Station builds Ships at ×0.75 Materials with no launch Emissions. Society **Charter Colonies**: a Colony with a Relay adds 1 to the Allotment per 4 Colonists.

**Why:** it gives the tree a late game where the board changes shape, so the Research Lead's pick becomes a strategic fork rather than a shopping order worked through in a fixed sequence.
**Size:** Large.
**Risk:** amends the `CONTEXT.md` definition of Tech, which currently promises only outputs, capacities, upkeeps, Ship strength, Influence costs and Emissions. At 60 each these land around turns 18–22 of 24 and need a sweep against the 0.02 climate clock. And every unlock needs an `ai.toml` weight — the AI already builds no Bank, Trade Post, Relay, Carrier or second station, so a new Module will sit unbuilt unless it is weighted deliberately.

## 4. The Lead's Prototype

When a Tech completes, the Research Lead also takes a **Prototype** of it: a second effect printed on the Tech's row, held by that Faction alone and only until the next Tech completes. Deep Mining's Prototype is a further ×1.25 on the Lead's Mines; Hardened Hulls' is +1 more Ship strength; Green Consensus's gives the Lead its Influence thresholds at ×0.6 rather than ×0.75; Efficient Transit's lets the Lead's Ships turn back mid-transit. Nothing about shared tech changes — the Tech itself belongs to everyone the moment it completes — so the Prototype is a lead, never a monopoly, and it expires precisely when the rival might have caught up.

**Why:** it makes the Research race worth winning turn by turn instead of merely conferring the right to pick, and gives a trailing Faction a reason to pour points into a Tech it did not choose.
**Size:** Small.
**Risk:** compounds with the Custodians' Research ×1.25 — they will hold most Prototypes most of the game. Either narrow the Research multipliers to 1.15/0.85 or make the Prospector-facing Prototypes (Deep Mining, Automated Refining) the sharper ones.

## 5. Discoveries buried in the Colony Slots

Give every Colony Slot a face-down Discovery dealt at setup from a deck of about twenty, revealed when a Colony is founded there or by a **Survey** order — a Ship in orbit at that Body stays stationary for a turn and spends 2 Fuel to turn one slot face up. A Discovery is permanent and local: "Lava Tube: Habitats here hold +2 and pay no Energy", "Regolith Ore: Mines here ×1.5", "Buried Ice: Refineries here ×1.5", "Barren: Generators here ×0.5", and "Nothing of note" on about a third of the deck. Rich Seam and Ice Deposit stay in the Event Deck as temporary bursts; these are the standing facts of a place, and they read on the Body Surface Map beside the slot's real name.

**Why:** choosing which named slot to found in becomes a decision with information you can pay for, and unloaded Colony Ships and idle Frigates get a peaceful job that is not waiting.
**Size:** Medium.
**Risk:** raises the reward for arriving first, which the AI already wins on Mars; a cheap Survey may push the Prospectors' extraction pace ahead of the climate clock tuned in 0.02. Also every Colony Slot is now a real place with a real name, so a Discovery that contradicts geology (Buried Ice at Olympus Mons) will read as wrong — the deck should be dealt per Body from a Body-appropriate list.

## 6. Divert Research: the Black Project

In Orders, a Faction may divert any whole share of this turn's Research away from the shared Tech into its own **Black Project**, at two diverted for one banked. Banked Research buys from a short private list — a sixth branch of four Techs no other Faction can ever hold — while everything diverted is Research the worldwide Tech does not receive, so the rival watches the shared progress bar slow and the tech panel reads "Prospectors diverting". Diverting also forfeits the Research Lead for the Tech in progress, because contribution is counted on what actually reached it. The private list should be things the shared tree would never offer: a Tech that lets Ships repair in orbit without a Shipyard, one that hides your Ship stack strengths from the rival's panels, one that raises the challenge margin against you by 10.

**Why:** it is the "should tech be shared?" question made playable — every turn the player chooses between the commons and a secret, and pays for the secret in public.
**Size:** Large.
**Risk:** contradicts the Research glossary entry outright ("flows automatically and entirely into the one Tech... cannot be saved or spent elsewhere"), so that entry has to be rewritten. Needs its own branch, its own panel and its own AI rule. The 2:1 rate is the one number the whole idea lives or dies on, and in a 24-turn game with twelve Techs there may simply not be enough Research for anyone to defect.

## 7. The Deck Census, and a deck that darkens

Put the Event Deck on the Climate Panel as a readable object: cards left and the count by kind — "14 left: 5 Climate, 4 Failure, 3 Solar, 2 Discovery" — never card identities. Then restore the retired 0.02 warming rule in additive form: in each Climate phase, for every full 0.2 °C above +1.2 not already counted, insert one new Climate card at a random position among the cards still in the deck. Because the deck is never reshuffled and only about fifteen of its twenty-eight are drawn in a game, the composition the player reads is genuinely the composition they face, and their own Emissions are visibly writing it.

**Why:** the deck becomes something the player watches and influences rather than something that happens to them, and warming acquires a second legible cost beside the Draw Chance.
**Size:** Small.
**Risk:** the Draw Chance was introduced in 0.02 specifically to *replace* deck-composition tinkering. Running both means warming raises frequency, damage and darkness at once, in a late game that already ends in Collapse in 17 of 20 AI seeds. If both run, the Draw Chance slope probably has to flatten.

## 8. Opportunity cards you have to reach

Four or five cards that hand out nothing and instead open a window in a named place for a named number of turns, claimable only by a Faction already positioned for it. "Derelict at Deimos: the first Faction with a Ship in orbit at Deimos at the end of this turn or the next takes 40 Materials." "Open Frequency: the first Faction to spend 10 Influence on the Middle East this turn doubles its Standing there." "Solar Maximum Window: a Colony that already has a Generator and builds another this turn pays half." If nobody is in reach, the card lapses and says so.

**Why:** good fortune becomes something you prepared for rather than something handed to you, which is the only honest way a positive card belongs in a game about consequences.
**Size:** Medium.
**Risk:** the AI's greedy single-pass scorer (16.1) has no notion of a window that stays open across turns; without an explicit hook it will never claim one, and a card only the human can use is worse than no card at all.

## 9. Crash Programme: Research as a spend

Let a Faction buy Research in Orders at 3 Ducats or 4 Energy the point, capped at its own Lab output that turn — so a Faction can at most double its science, and a Faction with no Research Labs can buy none. Bought points are contribution like any other and count toward the Research Lead. The cost sits in the same purse as everything the Trading window sells: Influence at 2 Ducats, a Restoration step at 20, Materials at 2, so one turn's Ducats go to exactly one of diplomacy, climate or science.

**Why:** Research is the only system in the game with no decision in it; this puts it into the same purse as the others and makes the last few points of a Tech something you can sprint for when the Lead is worth having.
**Size:** Small.
**Risk:** both the Custodians' Research ×1.25 and their Allotment ×1.3 scale from money, so this widens an already-noted gap; and the Prospectors' Research ×0.75 makes buying strictly worse for them, which needs either a flat rate applied after multipliers or a Prospector-facing reason to buy at all. The cap must be measured, not guessed.

## 10. The world watches the pick

Tag every Tech green or industrial on its row. When a Tech completes and the Research Lead picks the next, the pick is a public act: an industrial pick adds 3 to the picker's Standing in every Nation State at Industry Level 3 or above and subtracts 3 in every state with Education Level 1.3 or above; a green pick does the reverse. It applies in Resolution (d) of the turn the Tech completes and never drives a Standing below zero. The tech panel shows the swing before the pick is confirmed.

**Why:** the Research Lead's pick becomes a diplomatic act with a constituency, so a Faction can want the Lead for reasons that have nothing to do with the Tech it would choose.
**Size:** Small.
**Risk:** eight states with fixed Industry and Education figures makes the swing nearly deterministic, and the AI's fixed pick order (16.4) will produce an identical Standing pattern every game. The tags must cross-cut the branches — some green Industry Techs, some industrial Society ones — or this just re-labels the Custodian/Prospector split and rewards nobody for anything.

## 11. Cards that seed cards

Let an Event card, on resolving, insert one named follow-up at a random position among the cards still in the deck. Permafrost Thaw seeds **Methane Plume** (+5 Emissions in the turn it is drawn, Temperature-scaled). Reactor Leak seeds **Evacuation** (that Colony loses 2 Colonists and its Influence threshold falls with them). Unrest seeds **General Strike** (that state produces nothing and its Standing Army does not replenish, for two turns). Rich Seam seeds **Seam Exhausted** (that Body's Mines ×0.75 for three turns). The seeded card is shown to both Factions as it goes in, and the Deck Census counts it, so the player knows the shadow exists without knowing the turn.

**Why:** the deck gains a memory, so a bad turn casts a shadow the player can plan around — the difference between a random game and a consequential one.
**Size:** Small.
**Risk:** a growing deck fights the "about fifteen cards in twenty-four turns" pacing 0.02 tuned deliberately, and chains that seed during a late-game Climate spiral stack on each other. Cap each seeded card at one live copy, and do not let a seeded card seed another.

## 12. Targeting by the board, not by the die

Version 0.03 cut the two cards that singled out a Faction, correctly: being picked on at random is not a decision. Bring the shape back with the target chosen by a measurable board figure instead. **Regulatory Backlash** hits the Faction with the higher Emissions from sources it controls (its most industrial state's Facilities make nothing at the next Income). **Rushed Schedule** hits the Faction that has built the most Ships in the last three turns (one Ship due this turn completes next turn instead). **Brain Drain** hits the Faction with fewer Research Labs (2 Standing per Lab drawn toward the other in the states where both stand). Ties go to nobody and the card lapses; the popup names the figure that chose.

**Why:** it restores faction-facing cards without the unfairness, because the card is the consequence of a measurable thing that Faction chose to do.
**Size:** Small.
**Risk:** directly reopens a decision made on ticket #32 and would need that ticket amended, not just this rule added. A board-chosen target is predictable enough to game — which may be the point — but the Prospectors' entire plan is high Emissions, and being punished twice for the same choice will feel less like consequence and more like a tax.
