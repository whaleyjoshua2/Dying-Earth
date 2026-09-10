# Suggestions: conflict, combat and military play

Brainstormed by an Opus agent on 2026-09-09 after reading the spec, the glossary and the playtest note. Proposals only; nothing here is decided.

## Where the agent thinks the military layer stands

The combat algorithm in section 10 is good and nobody can see it working. More importantly, nothing in the game currently *wants* a war: the two Victory Conditions are economic, the Collapse Line means a wrecked Earth is a mutual loss, and Influence takes a Nation State more cheaply than an Army does. Version 0.04's open item 2 records the symptom — "the Prospector AI has never raised an Army in any batch". The agent's opinion is that **the Cold War should be the default texture and the hot war the expensive exception**: give war a price in Standing and in the Climate Model, give fleets a job that isn't killing (blockade), and give the player a covert middle path. The suggestions below are ranked on that reading; 1, 3, 4 and 12 build the cold war, 2, 5, 6 make the hot one about strangling and holding rather than annihilating.

---

### 1. Condemnation: aggression costs Standing

Every Battle a Faction starts — a stack set to Attack, or an Army entering a neutral or enemy Nation State — costs it Standing everywhere, applied in Resolution (d) before decay: −5 on every Nation State it does not control, −2 on every one it does, doubled if the target was a neutral state, halved off Earth. The Battle Report names the price in one line ("Condemned: −5 Standing in six states"). It is paid in the currency that actually decides the game — the challenge margin of 10 from version 0.04 means five points of Standing is roughly half a contested state — and each Faction has its own recovery: the Custodians' ×1.3 Allotment, the Prospectors' Ducats at two for one. The precedent already exists in the Unrest card, which drops every Faction's Influence in a state by 5.
**Why it's better:** it turns "take Europe by force or by money" into arithmetic the player can do, and makes the first shot the most expensive one.
**Size:** Small.
**Risk:** set too high it makes Earth war strictly dominated by Influence and the whole military layer stays dead; the numbers have to leave conquest *faster but poorer* than persuasion. Needs a visible running total or the player will feel punished by something invisible.

### 2. Blockade starves a Colony

Extend Orbital Control from a landing gate into a siege. A Colony or Space Station at a Body where a rival holds Orbital Control and its own Faction does not is **Under Blockade**: its Modules produce nothing at Income (Energy upkeep is still paid, so it can drag the Stockpile down), no Colonists load or unload, and after three consecutive turns of unbroken Blockade it loses one Colonist a turn. Breaking it means bringing a Frigate or Battleship and clearing the orbit. Because Off-world Presence is twelve Colonists in Habitats off Earth, a blockade attacks the rival's Victory Condition directly without a single Army landing.
**Why it's better:** it gives warships a purpose that no other unit can serve, and it is the only way to hurt a rival's colonial position without the Condemnation of invading.
**Size:** Medium.
**Risk:** one Frigate parked at Mars shutting down a whole colonial economy is a runaway; it wants suggestion 6 as the counter, and possibly a rule that a Body with more Colonies than the blockader has warships is only half-blockaded.

### 3. Covert Action spent from Standing

Let a Faction spend accumulated **Standing** (not the Allotment) at a place it has Standing in but does not control, choosing one operation per place per turn: **Sabotage** (10 Standing — one Facility or Module offline until the next Resolution and a 1-in-4 destruction roll), **Theft** (10 — take 10 Materials or 15 Ducats from the holder's Stockpile), **Incite** (15 — the state's Standing Army takes 2 damage and the controller's Standing there falls 5), **Delay** (10 — one build there completes a turn later). Each rolls **Exposure** at 1-in-3, reduced to 1-in-6 by a standing Embassy or Relay at that place: on Exposure the operation still happens, the Report names the actor, and the actor pays the Condemnation of suggestion 1.
**Why it's better:** it gives a Faction that will not fight a way to fight, and makes Standing a thing you can spend down rather than only bank — which the persistent Standings of 0.03 currently do not allow.
**Size:** Medium.
**Risk:** it competes with Influence for the same pool, so Standings may stop reaching thresholds at all; and "covert" is strange in a game where both Factions order simultaneously against one board, so the Report has to be disciplined about what the victim is told when Exposure fails.

### 4. The smoke of war

War emits. Each Battle fought in a Nation State adds Emissions in the next Climate phase — 1.0 per engaged Army plus 2.0 for every Facility the destruction roll of section 8.5 destroys, methane-weighted for a Refinery — charged to the Faction that attacked, and shown as a "War" line on the Climate Panel beside Factories and Launches. The other half is the sting: a destroyed Factory or Refinery stops emitting its 1.0 or 1.5 *forever*, so a Custodian who bombs the Prospectors' Refineries can buy a Stabilization run with someone else's industry.
**Why it's better:** it makes the Climate Model a party to every war, and hands the Custodians a strategy that works and that they should be ashamed of.
**Size:** Small.
**Risk:** bombing-for-Stabilization may be simply the best Custodian line. Whether war Emissions count against a Stabilization run — the way card Emissions explicitly do not (section 13.2) — is the decision that settles how strong it is, and it's yours.

### 5. Insurgency in an occupied state

While a Nation State is Occupied, each Income it raises a **Partisan Army** — strength equal to its population divided by 10 rounded up, capped at 3, Hit Points 5 — which attacks the occupier's Armies in Resolution (b) every turn and never leaves the state. Each turn of Occupation also costs the occupier Energy equal to the state's population factor, and while any Partisan Army stands the Pacification gain (one third of the threshold per turn, section 8.5) is cut by a third.
**Why it's better:** occupying Asia at 43.5 population and Australia at 0.5 cost exactly the same today; this makes the size of a conquest the price of it, which is the single most board-game-legible way to price aggression.
**Size:** Medium.
**Risk:** it may make occupying the big states never worth it, collapsing conquest onto the small ones. The three-turn transfer cap may need to be the pressure valve — perhaps partisans lengthen the count rather than only slowing Pacification.

### 6. Ground batteries that deny orbit

A new Module, the **Battery** (25 Materials, 1 turn, 3 Energy upkeep, Strength 4, Hit Points 6, never moves), and its Facility twin on Earth. It fires at enemy Ship stacks at its Body in Resolution (b) as though it had Stance Hold, and — the point — it **denies** the enemy Orbital Control while it stands, without granting Orbital Control to its owner. A Colony with a Battery cannot be blockaded (suggestion 2) until the batteries are shot down, and a landing needs the orbit truly clear.
**Why it's better:** a Faction that builds rather than fights gets a way to be hard to hurt, and a blockade becomes a siege with something in it to shoot at, rather than a fleet floating over an inert rock.
**Size:** Medium.
**Risk:** the deny/grant distinction is load-bearing — if a Battery granted Orbital Control, defended Bodies would make fleets pointless. It also adds a third thing competing for a Colony's Energy alongside the Barracks.

### 7. The Battle Report played out

Turn the Battle Report line into something you watch. For each Battle the Report popup shows the two stacks as rows of unit cards with damage pips, then the three rounds of section 10.2 as three dice each in the winning side's colour, each hit flying to the unit that took it, a disengaging unit sliding out of the line with the pursuer's d6 chasing it, then the survivors and the Occupation or Orbital Control that resulted. A Skip button and the existing text summary underneath. Every number it draws already exists in the algorithm.
**Why it's better:** the combat maths is the most carefully specified thing in section 10 and it is currently invisible; drama in a board game is watching the dice land, and this is the cheapest drama in the list.
**Size:** Medium (pure interface).
**Risk:** section 17.7 says no animation beyond the globes turning and transits creeping. This is a deliberate exception to a stated style rule and needs your yes.

### 8. Veterans earned in the Battle Report

A unit that survives a Battle in which its side landed at least one hit gains a **Veteran** step, to a maximum of two: each step is +1 Strength for any unit and +1 Pursuit for a Frigate. Steps survive repair and are lost only with the unit. They show as pips on the stack panel, the roster and the Battle Report, and a veteran unit is named in the Report when it fights again.
**Why it's better:** nothing today gives you a reason to pull a damaged ship out alive instead of trading it, which means Evade and Disengage — two well-built rules — are never chosen; a two-step veteran Frigate is worth running away with.
**Size:** Small.
**Risk:** mild snowball, and Strength inflation stacks with Hardened Hulls' +2 against a battle that is only three rounds long; the cap of two and a check on the odds preview (10.4) are what keep it honest.

### 9. Refits at a Shipyard

At a Shipyard a Ship may spend one turn stationary and 20 Materials — or 40 Ducats through the trading window's building rate — to install one **Refit**, replacing any it already carries: **Armour** +3 Hit Points, **Guns** +2 Strength, **Engines** +2 Pursuit, or **Tanks** transit Fuel ×0.75. The order sits beside Repair in the stack panel and uses the same stationary-at-a-Shipyard condition from section 5.3. One Refit at a time, swappable.
**Why it's better:** version 0.04 made Shipyards the only source of Ships and then gave them nothing to do once the fleet exists; this lets a small fleet be shaped to a job instead of out-built, which suits a game where Materials are always short.
**Size:** Small.
**Risk:** with only four Ship types, refits blur them — a Guns-and-Armour Frigate approaches a Battleship at half the cost. One-at-a-time exclusivity is what preserves the type identities, and the numbers should stay short of the gap between the types.

### 10. Mercenaries hired with Ducats

Add to the trading window a **Mercenary Army** and a **Mercenary Frigate**, 60 Ducats each, appearing immediately with no build time at a controlled Nation State with a working Launch Site or at any of the Faction's Shipyards. Each costs 10 Ducats every Income and disbands at once if unpaid. Mercenaries carry double Condemnation (suggestion 1) and cannot begin an Occupation on their own — they beat the defenders, but the count starts only if a regular Army is present.
**Why it's better:** it gives Ducats a war-chest use beyond buying Influence, and gives a Faction one turn's notice to answer a threat, which two-turn build times otherwise make impossible.
**Size:** Small to Medium.
**Risk:** version 0.04's open item 3 already notes the trading window has no caps and a rich seat can turn one turn's Ducats into forty-odd Materials; the upkeep is the price but a hard cap on mercenaries in play, or a price that rises with each one hired, is probably needed.

### 11. Neutral states arm when threatened

A neutral Nation State's Standing Army cap rises above its Industry Level + 1 by one for each of: an enemy Army standing in an adjacent continent, either Faction's Standing there above half its threshold, and every third turn since the game began — up to Industry Level + 4. It still replenishes 1 a turn at Income. A neutral state that is attacked and holds raises its cap permanently by 1. The Earth Map's grey shield number rises visibly as it happens.
**Why it's better:** it puts a clock on conquest — the state you have spent four turns softening up with Influence is the same one digging in — and gives the world an opinion without needing a third AI to hold it.
**Size:** Small.
**Risk:** it may close late-game conquest entirely, which is fine if Influence is meant to be the main path but is a door being shut; and it makes the early rush strictly better than the patient build, which may not be the pacing you want.

### 12. Ultimatum or surprise attack

An attack on a place the rival Faction controls is ordered as one of two things. An **Ultimatum** is declared this turn, appears in both Factions' Reports, and resolves in the *next* turn's Resolution: Condemnation is halved and the defender has one full Orders phase to reinforce, evacuate or counter-spend Influence. A **Surprise Attack** resolves this turn, gives the attacker one extra hit-roll in the first round only, and doubles Condemnation. Both appear on the Attack button beside the existing odds preview.
**Why it's better:** it makes opening fire a public act with a stated price instead of a button, and a declared Ultimatum that the defender answers by reinforcing is exactly the cold-war beat this game's fiction wants.
**Size:** Small.
**Risk:** against a single AI opponent an Ultimatum is only interesting if the AI reacts to it; section 16.3's threat multiplier would need to read declared Ultimatums as inbound stacks, or the option is a pure discount for the player.

---

## Two things to decide before any of this is built

- **The AI has never raised an Army** (0.04, open item 2) and builds no Carrier. Every suggestion here is theoretical until the base weights in 16.2 and the threat multiplier in 16.3 make the AI fight; suggestions 1, 4 and 5 in particular change the value of war and will need weights of their own. The agent would sequence a small AI-aggression pass before or alongside whichever of these is picked.
- **`CONTEXT.md` says the Custodians and Prospectors "differ in everything but combat."** Several of these (Condemnation recovery through the ×1.3 Allotment, mercenaries through Ducats, bombing-for-Stabilization) create *de facto* combat asymmetry without changing the strength tables. Whether that sameness is a rule to keep, or a gap to fill deliberately — Custodians better at Covert Action, Prospectors' Cheap Industry extending to hulls — is a design call left open.
