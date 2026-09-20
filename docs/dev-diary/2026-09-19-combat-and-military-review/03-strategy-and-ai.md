# Strategic role of force, and how the computer seats wield it (read-only review, 0.08.4)

*Filed from the review session of 2026-09-19 (evening), where a general-purpose agent wrote it as a read-only report against `main` at version 0.08.4, with a brief that said: cite every rule as file:line, mark every figure from a sim log as measured behaviour, never run the game bare, never present a change as decided. Proposals only; nothing here is decided.*

## 1. How force is used today

**Rules (code/data, cited):**

- *Units and prices* (`assets/data/units.toml`; `modules.toml:53-58`; `facilities.toml:73-78`): Army 25 Materials, 2 Energy/turn, strength 4, 5 HP; Frigate 25/2, str 3; Battleship 50/4, str 7; Carrier 30/2, unarmed, carries one Army; Barracks 20/2 (a Colony Army never leaves, `orders.rs:1079-1081`); Constabulary 25/2. No Ducats are spent on war anywhere. A march costs nothing and needs only adjacency (`orders.rs:1070-1088`).
- *Every Region has a Standing Army* (`state.rs:1265-1268`), strength = Industry Level + 1 (`state.rs:1984-1998`), fighting for whoever controls the Region, standing down under Occupation. A destroyed one is **raised again next Income at strength 1** and heals 1/turn (`economy.rs:207-230`). So no Region is ever undefended for more than the turn it was emptied.
- *Battle* (`combat.rs`): a melee, at most 3 rounds of 3 hit-rolls, disengage chance = damage/HP/2. First-round odds `p^3+3p^2(1-p)` with p = A/(A+D) (`combat.rs:135-142`). One Army (4) against a fresh Standing Army in an Industry-2 Region (3) is 61%.
- *Occupation* (`resolution.rs:545-608`; `influence.toml` `occupation_turns=3`, `destruction_chance=0.25`; `unrest.toml` `occupation_start=3`, `occupation_per_turn=1`): needs an Army with Stance Attack, not escaped, alone at the place; transfers after 3 turns, or at once if the occupier's Standing already meets the threshold ("Pacified"). Every building at an attacked place rolls 25% to be destroyed, again on transfer. An occupied Region's Labs pay the world, not the occupier (`economy.rs:243-246`); an occupied Colony's Archive is offline (`economy.rs:505-522`) and Scrubbers/Archive are destroyed on change of hands (`resolution.rs:673-695`).
- *Relations*: opening a Battle costs 3 rungs with every Faction on the other side and **breaks a non-aggression Accord outright** (`resolution.rs:346-352, 366-376`; `state.rs:3367-3383`). Attacking a neutral Region's Standing Army offends nobody. The `Passage` Accord term has no rule behind it anywhere (`state.rs:932, 3562` only).
- *Victory* (`victory.rs:113-170`, `factions.toml` cards): **no Victory Condition needs, counts or rewards force.** Force can *deny* one: an occupied Colony's colonists count for the previous controller until transfer, then for the taker (`state.rs:38-44, 2368-2372`), so Off-world Presence and Diaspora can be taken; the Archive is destroyed on transfer (fund kept); Scrubbers die with a Region, which can end a Stabilization run; Investment Banks are 25%-roll casualties (seed 1: two destroyed at Nigeria).

**AI appetites (rules, `engine/src/ai.rs`, `assets/data/ai.toml`):**

- Categories: `Warship`, `ArmyOrBarracks`, `Constabulary`, `StanceAttack/Intercept/Hold/Evade`, plus Carrier under `LoadUnload` (`ai.rs:10-63`). Weights (Prospectors/Custodians/Arkwrights/Archivists): warship 5/3/3/1, army-or-barracks 5/4/4/4, stance_attack 4/2/2/2, against found_colony 9 and influence 5-8. Score = base x gap x threat x opportunity (`ai.rs:81-83`); the victory-gap multiplier (up to x3) never touches a military category, and ticket #50 removed the denial multiplier (`ai.rs:65-66`), so a military candidate can never score above ~7.5 (Prospectors' march: 4 x (1 + (industry+size)/7)) while a Colony Ship scores 27.
- Army building is capped at **fewer than two** non-standing Armies per seat (`ai.rs:789-791`); `threat` x2 applies only when an enemy Army is adjacent (`ai.rs:236-252, 788`).
- Only the Prospectors may attack a Region they did not lose, march on neutrals, or attack in orbit on odds alone (`ai.rs:1841-1847, 1881-1886, 1900-1911`); the other three fight only to retake a place taken from them. Only the Prospectors ever want a Carrier or load an Army (`ai.rs:441-453, 1781`).
- Before attacking the AI reads **odds only** (`attack_odds=0.6`). It reads Relations solely for Agitate/Smear (`ai.rs:1270, 1297`) and never checks `accord_has`; it never reads Unrest or Standing for a war target. It "defends" only by Influence holds (`ai.rs:1084-1106`) and by Hold/Evade stances; a stack at 2/3 damage Evades at x10 weight. It will attack a player-held Region on the same odds rule (Prospectors) but the target is chosen by value, not by rivalry.

## 2. Measured

Seeds 1-3 (`sim --log`, seat 0 Custodians; Prospectors won all three on turns 33, 28, 28):

| | seed 1 | seed 2 | seed 3 |
|---|---|---|---|
| Battles | 6 | 3 | 0 |
| Attacker | Prospectors only | Prospectors only | - |
| Defender | Standing Armies of Custodian/Arkwright Regions | Custodian Regions | - |
| Occupations begun / completed | 2 / 1 (Russia, "Pacified" at once) | 0 / 0 | 0 / 0 |
| Places taken by Influence | 18 | 21 | 16 |
| Armies built / destroyed | 4 / 1 | 3 / 0 | 2 / 0 |
| Warships built (all Prospectors) | 2 | 4 | 0 |
| Space battles | 0 | 0 | 0 |
| Materials made per game (Cu/Pr/Ar/Arch) | 744/2721/149/278 | 616/2409/153/322 | 297/1798/104/256 |

Roughly 3 battles per 36-turn game here; the code's own earlier count was 55 battles over 80 games, 46 of them against neutrals (`resolution.rs:366-368`). The filed sweep (`docs/dev-diary/2026-09-19-version-0.08.4/sweeps/final-0.08.4.txt`) prints **no military line at all** — it reports 285-387 places taken by Influence per seating, 130 Accords struck, 117 Cold-or-worse pairs, but never a battle, Army or Occupation. Warships were built and then shuttled Earth-Moon-Earth on refuel loops with no target (seed 1, Battleship 33).

Two measured defects: (a) seed 2 Egypt and Mexico — the log says "The Prospectors are alone at the place; Occupation begins" (`resolution.rs:378-382`, which ignores `escaped`) but no Occupation began because the attacker disengaged (`resolution.rs:577-582` requires `!a.escaped`); the Standing Army respawned and the AI then spent 16 turns of Influence on Mexico instead. (b) seed 1 Saudi Arabia — the AI marched its occupier on to Nigeria the next turn, ending the Occupation itself: nothing weights *staying*.

## 3. Why nobody fights

1. **No prize only force can take.** Every Victory part is reached by building, banking, researching or Influence; an Army wins nothing an Embassy does not, and Influence keeps the buildings (no destruction roll, `resolution.rs:716-719`).
2. **Force is priced like a Factory but weighted like a whim** — base 4-5 and never gap-multiplied, so it loses the greedy sort every turn (3,799 "save ... build Army" lines in seed 1 alone).
3. **The defender regenerates for free** every turn; the attacker pays 2 Energy a turn and a 3-turn wait, then rolls 25% to wreck what it came for.
4. **Three of four Factions are forbidden to attack** by rule, and the Prospectors — the one seat that is allowed and can afford it (1,800-2,700 Materials a game against the Arkwrights' 104-153) — already win 13-15 of 20 by the Fund without it.
5. Nothing connects war to the rivalry systems: the AI never chooses a target by Relations, never offers or fears an Accord because of Armies, and Blame/Unrest never flow from war except the occupation Unrest that punishes the *taker*.

## 4. Proposals (ranked)

1. **Casus belli, war aims and peace (goal structure) — medium lift.** Rule as it would read: a Faction may *declare war* on a rival only with a cause the game recognises (Cold or worse Relations; an Agitate/Smear against you this turn; a rival at three-quarters of its Victory, i.e. the ticket #261 Moment; a Region of yours taken by Influence within N turns). A declaration names a **war aim**: one Region or Colony. While at war, Occupation of the aim completes in 1 turn instead of 3, buildings there roll no destruction, and Battles between the belligerents cost no further Relations. Peace is an Accord (non-aggression) either side may offer once the aim is held or 6 turns have passed; taking anything *beyond* the aim is the griefer case and costs a scar step per Region. Hooks: Relations rungs, the rival's Moment, Accords, Smear/Agitate offences. Risk: the AI declaring on the player every game — gate the AI declaration on the same causes and on a shared "at most one war per seat at a time".
2. **A prize only force takes — medium.** Suggest: a Region's Investment Bank/Exchange/Archive-site yield transfers *with its stockpile* to an occupier (Fund balance pro rata, Archive fund quarter, Scrubber count), and a taken-by-force Region keeps its Standing (not reset). Today the only thing force does that Influence cannot is *destroy*; give it something to *gain*.
3. **Make the AI a credible, visible threat — small/medium.** Add a `war_footing` multiplier on `ArmyOrBarracks`/`Warship`/`StanceAttack` that switches on when the seat is Cold or worse toward a neighbour or when a rival's Moment fires, and let all four Factions march (odds rule unchanged) — the Custodians on Regions whose Scrubbers a rival destroyed, the Arkwrights on a Body where a rival blockades. Visibility: an Army built or moved adjacent to a player Region is already a Report line; add a "musters against you" line when war_footing is on. Also raise the `armies < 2` cap to scale with Regions held. Risk: griefing the weakest seat — cap AI targets to Regions of the seat it is Cold toward.
4. **Fix the two measured defects — small.** (a) Make `army_melee` and `resolve_occupation` agree: either an escaped-but-alone attacker occupies, or the line stops promising it. (b) Weight "stay and hold the Occupation" (a Hold candidate at the march's score while `Occupied{occupier==seat}`) so the AI does not abandon Saudi Arabia for Nigeria.
5. **Standing Army respawn delay — small.** A destroyed Standing Army returns after `unrest.army_threshold`-style delay of 2 turns rather than the next Income, so a won Battle opens a real window. Risk: makes neutrals easy meat for the Prospectors; pair with proposal 1's cause requirement for neutral Regions (a neutral's Standing Army defending offends "the world": +Blame or Unrest in neighbours).
6. **War and Unrest/Blame — small.** Occupation Unrest (+3, +1/turn) should also raise Unrest in the *attacker's* own Regions by a fraction (war weariness) and lay a Blame ppm figure on the aggressor (a "brutality" ledger reusing the Smear ledger, `influence.toml [smear]`), so the climate clock and the Custodians' credit market feel the war.
7. **Give Passage a rule — small.** Armies may march through a partner's Region only under a Passage Accord; without it, marching into a rival's Region is itself an offence rung 2. This makes the Accord term real and gives the player a diplomatic gate on ground war.
8. **Carriers off Earth — medium.** The only Carrier logic is the Prospectors' (`ai.rs:441-453`). If proposal 2 makes an off-world Colony worth taking (Diaspora denial), extend `wants_carrier` to any seat at war and let an Intercept stance actually fire (no space battle in 3 seeds; the Custodians' rule only breaks blockades).

## 5. Open questions for the designer

- Should war be *declared* (proposal 1) or stay an implicit act of marching? Declaration is what makes it visible and gives peace a shape; it is also the larger lift.
- Which prize should force uniquely take: the Fund/Archive stockpiles (2), the Presence count via Colonies (already a rule), or Regions with buildings intact?
- May the AI ever start a war against the *player* on causes alone, or only in answer to the player's offences? (Griefer risk sits entirely here.)
- Should the three non-Prospector Factions be allowed to attack at all, and on what cause — or is pacifism part of their identity?
- The sweep needs a military line (battles, occupations, Armies built/lost by seat) before any of this is tuned; small lift, and I recommend it precedes the rest.
