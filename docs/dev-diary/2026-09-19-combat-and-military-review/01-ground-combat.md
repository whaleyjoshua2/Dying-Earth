# Ground combat and Armies in Dying Earth 0.08.4: a read-only review

*Filed from the review session of 2026-09-19 (evening), where a general-purpose agent wrote it as a read-only report against `main` at version 0.08.4, with a brief that said: cite every rule as file:line, mark every figure from a sim log as measured behaviour, never run the game bare, never present a change as decided. Proposals only; nothing here is decided.*

All paths are under `C:\Users\Josh\games\Dying-Earth\`. "RULE" = read from code or data, with a citation. "MEASURED" = from headless `sim` logs (seeds 1, 2, 3, all four seats AI, seat 0 Custodians in China) or the 0.08.4 sweep file. No code was changed and nothing was run except the headless `sim` example.

## 1. Rules as they stand

**Units (RULE, `assets/data/units.toml:64-73`).** One Army card: 25 Materials, 1 turn, 2 Energy upkeep, strength 4, hit points 5, pursuit 2. Repair is 5 Materials a point (`units.toml:76`). Moving an Army costs nothing (`engine/src/orders.rs:286` prices only BuildArmy). A built Army needs a Region you control (`orders.rs:923-928`); a Colony Army needs a Barracks (20 Materials, `modules.toml:54-58`) and each Barracks holds exactly one (`orders.rs:935-943`). Colony Armies never leave (`orders.rs:1079-1081`).

**Standing Army (RULE).** Every Region has one, free. Its strength is `industry_level + 1` minus damage (`engine/src/state.rs:1984-1997`), so 2 to 4 on today's board (`nation_states.toml`: levels 1-3), yet its hit points are the card's 5. It heals 1 damage a turn while Unrest is below 4 and the state is not Occupied (`economy.rs:207-229`, `unrest.toml army_threshold = 4.0`); destroyed, it is re-raised next Income at strength 1 (`economy.rs:211-219`). A Standing Army at strength 0 is not counted as a defender (`resolution.rs:321`). It fights for whoever controls its Region and stands down while the Region is Occupied (`state.rs:1965-1982`).

**Trigger (RULE).** A `MoveArmy` order to a neighbouring Region (`orders.rs:1082-1084`, adjacency from `nation_states.toml neighbours`) resolves as a move; entering a Region you do not control forces the Army's Stance to Attack (`resolution.rs:214-231`). A ground Battle fires at any place where a seat that does not direct the place has an Army at Attack (`resolution.rs:253-267`); everyone else there, including a neutral Region's own Standing Army, is a defender (`resolution.rs:284-296`). Hold is the default and does nothing; Attack on a place you direct does nothing; Evade is a 50% coin to leave before the first round (`combat.rs:176-182`). So for Armies the Stance order has effectively two live values.

**Resolution (RULE, `combat.rs:186-243`).** At most 3 rounds; each round exactly 3 hit rolls; each roll picks the hitter by share of total strength present and then a random engaged enemy unit, which takes 1 damage. After each round every damaged unit disengages with chance `damage / hp / 2` (`combat.rs:150-158`); leavers are chased by the best enemy pursuer on a d6 <= pursuit (an Army chases 1 in 3, `combat.rs:265-290`). There is no terrain, no supply, no morale, no combined arms, no fortification; the only inputs are strength, hp, damage and pursuit.

**Capture (RULE, `resolution.rs:578-611`).** If after the Battle the attacker has an Army at the place and no defender is left engaged, Occupation begins: +3 Unrest at once, +1 a turn (`unrest.toml occupation_start/per_turn`), the occupier gains Standing each turn by `pacification_gain` through Resistance (`resolution.rs:615-622`), and control transfers on Pacified (Standing >= threshold) or after 3 turns (`influence.toml:35 occupation_turns`). The occupation breaks the moment the occupier has no Army at the place (`resolution.rs:547-558`). A place taken by force rolls every Facility or Module for destruction at 25% (`influence.toml:36`, `resolution.rs:482-537`), and again on the attack itself (`resolution.rs:305`), and its Scrubbers are destroyed outright (`resolution.rs:654`).

**Escape (RULE).** A defender that disengages is removed from `defenders_at` for the rest of the turn (`resolution.rs:320`), so a Standing Army that runs hands the Region to the attacker that turn. `escaped` clears at the next Income (`economy.rs:134`).

**Ships (RULE).** A Carrier (30 Materials, strength 0, hp 4) carries one Army; it can only be loaded from a Region with a working Launch Site (`orders.rs:1155`), and a landed Army arrives at Hold (`resolution.rs:1888-1892`), so an invasion of a Colony takes a landing turn and then an Attack turn. Orbital Control shuts the ground only when one rival holds it outright (CONTEXT.md, Orbital Control).

**Diplomacy and cost (RULE).** Opening a Battle is a rung-3 offence against every party present (`resolution.rs:345-350, 370-375`); against a neutral Region's own Army it offends nobody. A non-aggression Accord does not refuse the order; the act breaks the Accord and still costs 3 (`state.rs:3371-3377`). Relations levels: Wary at -3 to -5, Cold to -8, Hostile below (`state.rs:3579-3581`). The Constabulary is not a combat building at all: it damps Unrest and adds 5 (10 with Civil Defense) to the Influence challenge margin (`influence.toml:16-32`). Blame is untouched by war.

**Report (RULE).** The only Moment combat can raise is DecisiveBattle, and it fires only from `destruction_rolls` when a building is lost (`resolution.rs:531`; the sole call site). A Battle in which an Army dies but no building burns is a Battle Report line, not a Moment.

## 2. Measured

Three seeds, all won by the Prospectors on turn 33/28/28:

| seed | Battles | Armies built (all Prospectors) | first Army turn | marches | Occupations begun | taken by force | taken by Influence | Armies destroyed | Carriers / Barracks built |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 6 | 4 | 26 | 5 | 2 | 1 (Russia, Pacified) | 18 | 3 | 0 / 0 |
| 2 | 3 | 3 | 22 | 2 | 0 | 0 | 21 | 1 | 0 / 0 |
| 3 | 0 | 2 | 25 | -- | 0 | 0 | 16 | 0 | 0 / 0 |

Across 9 Battles: 4 units destroyed, 7 Battles ended by an escape, 4 destruction rolls burned 9 buildings (an Investment Bank and a Scrubber among them). Every attack was a single strength-4 Army against a single Standing Army of strength 1-3 at odds the AI printed as 61% or 90%. Every Battle was Prospectors versus a rival's Standing Army; no seat ever defended with a built Army, no Army ever moved into a friendly Region, and in seed 1 the Prospectors abandoned their own Occupation of Saudi Arabia after one turn to march on Nigeria. The Custodians, Arkwrights and Archivists built no Army in any seed: the "build Army" candidate sat at score 4-5 and was passed over 70+ times a game for Power Plants and Trade Posts (log: `save 4.0 build Army in China (holding Materials for ...)`). One place was taken by force against 55 taken by Influence.

The 0.08.4 sweep (`docs/dev-diary/2026-09-19-version-0.08.4/sweeps/final-0.08.4.txt`) reports no Battle or Army figure at all; it counts 285-387 places taken by Influence per 20-game seating, 10-52 Constabularies, and 22-45 non-aggression Accords standing. The engine's own comment records 55 Battles over 80 games, 46 of them against neutral Standing Armies (`resolution.rs:368-369`). So MEASURED: about 0.7 Battles a game, versus 15-19 Influence takes.

## 3. Where it is weak

1. **Force is never the best way to take a place.** Influence keeps every building; force burns a quarter of them twice and adds +3 Unrest that the occupier must then Relief away. A built Army is 25 Materials plus 2 Energy a turn; 25 Materials of Influence at 5 a step buys the same Region without a Battle and without a rung-3 offence. The AI's `held_state_weight = 0.3` (`ai.toml:279`) is the honest reading: nobody attacks a held place unless nothing neutral is left.
2. **The Battle is a coin toss with no lever.** Two inputs (4 vs 2-4), three rolls a round, and the outcome is decided by disengage dice. Every fight in three games was 1 Army vs 1 Army. There is nothing to bring, nowhere to stand, and no reason to wait a turn.
3. **The defender's best move is accidental.** A Standing Army that disengages hands the Region over the same turn; a player cannot order it to stand, and the Region's Constabulary, Industry and population count for nothing in the fight.
4. **No decision after the first Battle.** The occupier just waits three turns. Reinforcement, siege, relief columns, a counter-attack from a neighbour: none of it happens because the AI has no appetite and the rules give no reason.
5. **Combat is invisible in the Report.** A Battle that kills an Army is not a Moment; only a burned building is.
6. **Barracks and Carriers are dead rules.** Zero built in three seeds; a Colony invasion needs a Launch Site, a Carrier, a turn to land and a turn to attack, against a garrison that can only sit there.

## 4. Proposals, ranked

**P1. A Region's defence is its people (small).** *Rule:* a Standing Army's strength is `industry_level + 1`, plus 1 while a working Constabulary stands there, plus 1 while Unrest is below 4; and its hit points equal its strength rather than the card's 5. *Why:* today a strength-2 Standing Army soaks five hits it cannot answer; this makes a calm, policed Region a real wall and a restive one a soft target, so Agitate becomes the opening move of an invasion. *Hooks:* Unrest, Constabulary, Agitate/Relief. *Lift:* small (`state.rs:1988`, `resolution.rs:332-340`, data). *Risk:* Level-1 Regions become easy pickings; the AI's attack odds must be recomputed with the new figures (it already calls `first_round_odds`).

**P2. Dig In: a third live Stance for Armies (medium).** *Rule:* an Army ordered Dig In adds +2 strength as a defender and never disengages, but cannot attack or move the same turn; a Region's Standing Army is always dug in. *Why:* gives the defender a decision, ends the "escape loses the Region" accident, and makes an attack want a second Army. *Hooks:* Stance, Report. *Lift:* medium (Stance enum, `combat.rs` disengage, AI stance appetite, tests). *Risk:* attacks become rarer still unless P3 or P5 land with it.

**P3. Take it whole (small).** *Rule:* the destruction roll on a place taken by Occupation is 25% only for buildings that were not Pacified in; a place that transfers by Pacified rolls nothing, and the roll on the attack itself is removed. *Why:* today force burns twice and Influence never; this makes "beat the Army, then win the people" a coherent plan and gives Occupation-then-Pacify a reason to exist. *Hooks:* Occupation, Pacified, Standing. *Lift:* small (`resolution.rs:305, 712-714`). *Risk:* the Prospectors, who already own war, may take one more Region a game; check with the sweep.

**P4. Plunder (small-medium).** *Rule:* the turn Occupation begins, the occupier takes Ducats equal to 5 x the Region's Industry Level from the Region (or from its holder's purse if held) and +1 Blame share for the burning. *Why:* gives an invasion a payoff other than the Region itself, which matters to a seat with 27 Ducats a game (the Arkwrights) and gives the rich seat a reason to fear one. *Hooks:* Ducats, Blame, Relations. *Lift:* small-medium (one rule in `resolve_occupation`, a Report line, AI appetite figure). *Risk:* rewards the AI Prospectors' existing habit of hit-and-run; pair with P6.

**P5. A Battle is a Moment (small).** *Rule:* any Battle in which a unit is destroyed or a place changes hands or is Occupied raises the DecisiveBattle Moment, with the odds the attacker faced and what each side lost. *Why:* the designer wants a remembered moment; today the engine only stops the turn for a burned Power Plant. *Hooks:* Report/Moments. *Lift:* small (`resolution.rs:378-385`, `report.toml`). *Risk:* none of substance.

**P6. Occupation must be held, or it breaks with a cost (small).** *Rule:* an occupier whose last Army leaves an Occupied place before it transfers hands it back at +2 Unrest and takes a rung-2 offence from the previous holder; while Occupation lasts the Army may not move. *Why:* the measured hit-and-run (Saudi Arabia, seed 1) is free today and makes war a nuisance rather than a campaign. *Hooks:* Unrest, Relations. *Lift:* small (`orders.rs:1070`, `resolution.rs:547`). *Risk:* the AI must learn to stay; a one-line change in the march candidate.

**P7. A second Army sails free (medium).** *Rule:* a Carrier lifts from any Region the seat controls (not only one with a Launch Site) and a landed Army may Attack on the turn it lands. *Why:* the Colony war does not exist; halve the friction and give the Barracks a job. *Hooks:* Carrier, Barracks, Orbital Control. *Lift:* medium (`orders.rs:1155`, `resolution.rs:1888`, AI carrier appetite). *Risk:* Archive Colonies become raidable; the Archivists have 0 wins in 80 already.

**P8. Muster from the people (large).** *Rule:* a Region raises Armies from its population (one unit of population a strength point) and Standing Army cap reads population and Unrest, so a depopulating Arkwright home cannot defend itself and a crowded India can field three. *Why:* ties war to the climate game's core resource, people; a Region emptied by Pioneers becomes a military question. *Lift:* large (new subsystem touching Pioneers, Unrest, AI, UI). *Risk:* deep; listed only because it is the one that would make Armies feel part of the same game as the rest.

Recommendation: P1 + P3 + P5 together as one ticket (all small, and they turn one Region-taking into a real alternative), then P6, then P2. P4 and P7 only if the designer wants war to be a Ducat and Colony matter; P8 is a version of its own.

## 5. Open questions for the designer

1. Should force ever be *cheaper* than Influence for a Region, or only a different path (buildings burned versus Standing spent)? P3 answers "different path"; a bolder answer would cut the 25% roll entirely.
2. Should a Standing Army be the Region's people (P1/P8) or stay a flat card figure? This decides whether Agitate is a military tool.
3. Is a Battle that only kills an Army worth stopping the turn for (P5)? Today the game says no.
4. Should the non-Prospector AIs raise Armies at all, or is war meant to be the Prospectors' signature? Today only they build them; the rule change reaches the computer seats either way.
5. Should there be a Region-level attacker cap, so a player cannot walk three Armies into a strength-2 Region (today unbounded, but measured never more than one)?
