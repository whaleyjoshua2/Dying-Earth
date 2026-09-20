# Ships, space and orbit: rules review for 0.08.4

*Filed from the review session of 2026-09-19 (evening), where a general-purpose agent wrote it as a read-only report against `main` at version 0.08.4, with a brief that said: cite every rule as file:line, mark every figure from a sim log as measured behaviour, never run the game bare, never present a change as decided. Proposals only; nothing here is decided.*

All paths under `C:\Users\Josh\games\Dying-Earth\`. RULES are read from code/data; MEASURED is from sim logs and the closing sweep.

## Rules as they stand

**Ship kinds** (`assets/data/units.toml:4-63`; every Ship's tank is 30 Fuel, paid at the build, `engine/src/orders.rs:285`):

| Kind | Materials | Turns | Str | HP | Pursuit | Upkeep | Carries |
|---|---|---|---|---|---|---|---|
| Colony Ship | 30 | 1 | 0 | 3 | 0 | 1 Energy | 4 Colonists (+crowd) |
| Frigate | 25 | 1 | 3 | 4 | 4 | 2 | nothing |
| Battleship | 50 | 2 | 7 | 8 | 1 | 4 | nothing |
| Carrier | 30 | 1 | 0 | 4 | 0 | 2 | one Army (str 4, hp 5) |

Only Frigate and Battleship are "warships" (`engine/src/ids.rs:416-418`). Hardened Hulls adds +2 strength to armed Ships only (`engine/src/state.rs:2000-2006`, `assets/data/techs.toml:109-116`). Repair costs 5 Materials or 10 Ducats a point, at a Shipyard/Launch Site, never in the same turn as a move (`units.toml:65-66`, `factions.toml:402`, `orders.rs:948-966`).

**When Ships fight.** A ship Battle happens in exactly two cases: (a) a stack ordered **Attack** pulls every other Faction's Ships at that Body into one melee (`engine/src/resolution.rs:184-207`); (b) a stack ordered **Intercept** fights every enemy stack arriving that turn (`resolution.rs:149-181`). Arrivals are forced to Hold (`resolution.rs:82`) and cannot transit again that turn (`orders.rs:1014`), so the defender always strikes first and an attacker never arrives swinging. Opening a Battle is a rung‑3 offence to every party and breaks a Non‑aggression Accord (`resolution.rs:345-351`, `state.rs:3367-3383`). Ships in transit are untouchable except by the Radiation Surge card (1 damage, `engine/src/events.rs:304-316`); a Solar Storm freezes every transit for a turn (`resolution.rs:71-76`).

**How the melee resolves** (`engine/src/combat.rs`). At most 3 rounds of 3 hit rolls (`:88-89`, `:188`), so **nine hits is the ceiling for a whole battle across all parties**. Each roll's hitter is drawn by strength share (`:110-116`), the target party by strength share, and the target *unit* uniformly among that party's engaged units (`:99-106`, `:195`). Disengage chance is damage/HP/2, Evade is a flat 50% at the start (`:147-155`, `:169-176`); a pursuer catches on d6 ≤ Pursuit and then hits at its strength share (`:260-297`). Consequences: a Battleship (8 HP) is effectively unkillable in one turn; two Frigates (6) against one Battleship (7) land 8 of 9 hits with probability ≈0.6%; a lone Frigate on Attack against a strength‑0 Colony Ship takes all three rolls with certainty and kills it in round one (3 HP), Colonists lost with it (`resolution.rs:446-458`); a Carrier destroyed destroys its Army (`:449-451`). Random targeting means an "escort" only halves the hits a Colony Ship takes; it does not shield it.

**Orbital Control** is held by the single seat with an unescaped warship at a Body; two seats' warships and nobody holds it (`state.rs:2789-2798`). Holding it outright shuts the *ground* to every rival's Colonists and Armies (`state.rs:2807-2812`, `resolution.rs:1497-1510`); contested orbit shuts nobody. It costs nothing to hold, needs no fight, and is neutralised by parity. **Blockade** is one warship sitting in one Orbital Slot, chosen blind with the leg (`orders.rs:1021-1027`, `state.rs:439-443`): it stops unloading into and refuelling at that slot's station only (`state.rs:2818-2852`). There is no bombardment, no interception in flight, no escort rule, no capture of Ships.

**Stations.** A station is a Colony; it may hold no Barracks (`orders.rs:838`), so `defenders_at` (`resolution.rs:317-324`) is always empty. An Army unloaded from a Carrier into a rival station (ownership is checked only for Colonists, `orders.rs:1200-1212`) starts an Occupation at once and takes it in 3 turns or on Pacification, rolling 25% destruction per Module (`resolution.rs:578-604`, `influence.toml:35-36`). The only defence is a warship blockading that slot.

**Fuel as a constraint** (`state.rs:2892-2919`, `bodies.toml`). Moon 6 Fuel (3 with Efficient Transit ×0.6), Mars 20 at the window rising 0.83%/degree off it (`ephemeris.toml`), Venus 16. Without Efficient Transit a 30‑tank Ship cannot cross to Mars more than 60° off the window (~±2 turns), arrives with 10 Fuel, and cannot come home (20) until it owns a 40‑Materials station there; so a Mars sortie is a one‑way commitment. Refuel is at an unblockaded station of your own only (`state.rs:1675-1681`, `orders.rs:1040-1046`).

**Accords.** The Passage and Refuel terms are *never read*: `Term::Passage`/`Refuel` appear only in the enum (`state.rs:932`), the acceptance gate (`state.rs:3562`) and a sim counter (`sim.rs:441`); `accord_has` is called only for NonAggression and ResearchAgreement (`state.rs:3375`, `:3568`). CONTEXT.md's Terms entry describes a rule the engine does not have.

## Measured

Closing sweep (`docs/dev-diary/2026-09-19-version-0.08.4/sweeps/final-0.08.4.txt`, 80 games): Refuel orders 155/123/106/55 per 20‑game seating; stranded Ships at the end [0,0,0,0] in three seatings, [2,0,0,0] in the Archivist one; stations off Earth 17/22/25/2; Accords standing 45/38/22/34, **passage 0, refuel 0** in all; Mars Colony in 4/6/5/2 of 20 seeds.

Sim seeds 11, 22, 33 (plus sibling seeds 1–5 in the same scratchpad, same build), 8 games:

| | Colony Ships | Frigates | Battleships | Carriers | Orbit battles | Ships destroyed | "orbit contested" denials | Refuels |
|---|---|---|---|---|---|---|---|---|
| 11 | 9 | 9 | 5 | 1 | 0 | 0 | 24 | 14 |
| 22 | 8 | 3 | 2 | 1 | 0 | 0 | 7 | 12 |
| 33 | 4 | 2 | 0 | 0 | 0 | 0 | 0 | 3 |
| 1–5 | 27 | 6 | 5 | 0 | 0 | 0 | 5 | 35 |

Totals: 32 warships built, 30 of them Prospector; **zero space battles, zero interceptions, zero Ships destroyed by any cause, zero Armies carried, zero Colonies or stations occupied from orbit**. The one story: seed 11, the Prospectors park two Frigates and a Battleship at the Moon from T21; the Archivists' two loaded Colony Ships are refused every turn T21–T33 (12 turns); T29 Tycho falls to Prospector Influence and its Archive is destroyed. The Prospectors' AI never generated an Attack candidate at the Moon though odds were 100%: Hold was taken 30/30 at weight 4.0 (`stance_hold` 2 × threat 2.0 equals `stance_attack` 4, `assets/data/ai.toml:56-65,199`), and the AI intercepts only when it already holds Control (`ai.rs:1852`). For the AI reviewer, but it is the proximate cause of the zeros.

## Where it is weak

1. Orbit is decided by presence, not force. One Frigate locks a Body's ground; a second Faction's Frigate unlocks it for everyone. Nobody needs to fire; parity wins.
2. The melee's nine‑hit ceiling makes ship fights indecisive unless one side is unarmed, and unarmed victims never appear because nobody attacks.
3. Passage and Refuel terms do nothing, so the Accord system has no naval currency.
4. Nothing to fight over: Mars is empty in 70–90% of games; the Moon (2 orbital slots, 1‑turn hop) is the only choke point, and its contest is a standoff.
5. A lost Ship costs Materials, Fuel and cargo but produces no Relations, Blame or Moment beyond a report line.

## Proposals, ranked

1. **Strength decides orbit** (small: `state.rs:2789` + AI + test). "Orbital Control is held by the Faction with the greatest warship strength at a Body; equal strength holds nothing." Parity stops being enough; a second Frigate or a Battleship becomes a real answer, and the losing side must fight or buy. Hooks: Relations (a lockout is an implicit offence?), Map. Risk: strengthens the Prospector lockout unless the AI's Hold/Attack tie is fixed in the same ticket.
2. **Enforce Passage and Refuel** (small: three predicates, `may_land`, `slot_blockaded_against`, `refuelling_station`, plus the Intercept filter). "Passage: the partner is neither shut out by Orbital Control nor by a Blockade, nor a target for Intercept. Refuel: the partner may Refuel at your stations." Makes the Moon key negotiable and gives the 130 Accords struck a game something to trade. Risk: AI must want them.
3. **Arrive with a stance** (small–medium: `Order::Transit` gains a stance, `resolution.rs:82`). "A Transit may carry Attack or Evade for its arrival turn." Timing a fleet's arrival against a window, or against a picket away refuelling, becomes a decision; the crossing is a planned blow, not a landing that waits a turn to be shot at.
4. **Escorts take the fire** (small: `combat.rs:99-106`). "While a party has an engaged warship, hits land on its warships; unarmed Ships are struck only when none remain." Makes a Frigate genuine insurance for a 30‑Materials, 8‑Colonist ship and makes the Carrier's "needs an escort" true. Risk: lowers lethality; pair with 5.
5. **Rolls per engaged warship, not a flat three** (small in `combat.rs:89,188`; shared with ground, so a ships‑only variant is the safer form). Lets two Frigates threaten a Battleship over two turns and makes Battleship‑versus‑Battleship a fight rather than a scratch.
6. **Prize rule** (medium). "A warship on Attack at a Body where a rival Ship is stranded and no rival warship stands may seize it with its cargo; a rung‑3 offence." Turns Fuel logistics into a hunt; the Ship‑name rule (prefix follows the holder, `CONTEXT.md` Ship name) already anticipates it. Risk: rarely triggers (0–2 stranded a seating) unless Refuel‑denial grows.
7. **Bombard** (medium–large). "A Battleship may Bombard a rival Colony at its Body: one Module rolls the 25% destruction; rung 3; Blame if it is a Scrubber/Solar Array?" Gives the fleet a coercive act without an Army. Risk: theme and griefing; the designer should rule on it before anyone charts it.
8. **Make a loss a Moment** (small, presentation reviewer's ground): destroyed Ship with Colonists or an Army aboard raises a Moment and a Relations deed for the loser's neighbours.

## Open questions for the designer

- Should an orbit be won by strength on station, by a fight, or stay a presence standoff?
- Passage is described in CONTEXT.md but unenforced. Enforce it, or strike the term?
- Stations fall to a bare Army in three turns with no defence possible. Intended?
- Are space Battles meant to be deadlier than ground ones, given one shared algorithm?
- Is the Moon the intended choke point for the whole game, with Mars empty in most seeds?
- Is bombardment or seizure in theme for this game at all?
