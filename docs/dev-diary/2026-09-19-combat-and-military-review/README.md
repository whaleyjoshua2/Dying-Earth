# 2026-09-19: four reviews of combat and military play

On the evening of 2026-09-19, with version 0.08.4 just merged, the designer asked for four agents to
review the combat and military mechanics "to improve them and make them more engaging and
compelling" and to report back for review. Each agent took one face of the military game, read the
glossary, the engine and the data, played headless `sim` games and read the 0.08.4 closing sweep,
and wrote a report with the rules as they stand (cited to file and line), what was measured (with
the seeds), where it is weak, five to eight ranked proposals with a lift size and a risk, and open
questions for the designer. **Nothing here is decided**; the lists exist so the designer can pick.

The reports were delivered in the session and are filed here unchanged, so the designer can read
them at leisure. This is the second time combat has been reviewed: the first was
[2026-09-09's `03-combat-and-military.md`](../2026-09-09-mechanic-suggestions/03-combat-and-military.md),
written before version 0.05 against a game with no Relations, no Accords, no Agitate and no named
Armies. Where the two agree across eleven versions, that is some evidence the weakness is structural.

| File | Angle | Proposals | Open questions |
| --- | --- | --- | --- |
| [01-ground-combat.md](01-ground-combat.md) | Armies, Standing Armies, Battles, Occupation and what a Region costs to take by force | 8 | 5 |
| [02-presentation.md](02-presentation.md) | What the player sees and orders, with eight headless pictures | 8 | 4 |
| [03-strategy-and-ai.md](03-strategy-and-ai.md) | Whether force matters to any Victory path, what war costs each seat, how the computer wields it | 8 | 5 |
| [04-ships-and-space.md](04-ships-and-space.md) | Frigates, Battleships, Carriers, Orbital Control, Blockade, Fuel, and the Accord terms that do nothing | 8 | 6 |

## Where the four reports agree

Agreement between agents working from different angles is some evidence an idea is load-bearing.
This is the session's reading of the four, not a decision.

- **Force takes nothing Influence cannot, and burns what it takes.** Every reviewer landed on this
  first. Influence keeps every building; force rolls a quarter of them for destruction twice, on the
  attack and on the transfer, and adds Unrest the taker must then Relief away. Three of the four
  Factions are forbidden by the AI's own rules to attack a Region they did not lose, and the one that
  may, the Prospectors, already wins 28 of 80 by the Fund without it.
- **The sweep is blind to war.** The closing sweep prints no Battle, Army or Occupation figure at all.
  All four say measure first, and the strategy reviewer puts a military line in the sweep ahead of
  every other proposal. Small lift.
- **Battles are invisible.** A Battle is not a Report line, so it cannot headline; the only military
  Moment fires when a building burns, never when a unit dies; a destroyed Army writes nothing at all;
  the Ship names of 0.08.1 and the Army names of 0.08.4 never reach the Battle Report because
  `run_melee` strips them; and the "61%" on an attack button is the chance of winning the first
  exchange, not the Battle, and the label never says so. Three of the four propose a Battle Moment.
- **Two measured defects worth a ticket on their own.** The Occupation that never began: the log
  promises "Occupation begins" where the attacker had in fact disengaged, because two functions
  disagree about `escaped`, and the AI then spent sixteen turns of Influence on the Region instead.
  And the AI's Hold/Attack tie: `stance_hold` 2 × threat 2.0 equals `stance_attack` 4 in `ai.toml`,
  so the computer's fleets never fire even at 100% odds, which is the proximate cause of zero space
  Battles in every game measured.
- **The rivalry systems just built are the natural hooks.** Agitate as the opening move of an
  invasion, if a Standing Army's strength reads Unrest and the Constabulary; a cause for war read
  from Cold Relations or a rival's Moment; the Passage Accord term made real, since today it is an
  enum value nobody reads; war laid on the Blame ledger the way Smear is.

## Ideas that came up more than once

- **A Battle is a Moment, and a loss is a Moment**: ground P5, presentation #2, ships #8.
- **A military line in the sweep**: strategy, explicitly first; ground and ships both note the
  sweep prints nothing and their own figures come from three and eight seeds.
- **Unrest and the Constabulary as the Region's defence**: ground P1, and strategy's hooks.
- **Passage as a rule rather than a word**: strategy #7 and ships #2, from opposite ends, one as
  an offence for marching through without it, the other as a key to a rival's orbit.
- **Escape is the accident that decides Regions**: ground weakness 3 and P2, strategy defect (a),
  ships' note that an arrival is forced to Hold and the defender always strikes first.
- **Occupation abandoned for free**: ground P6 and strategy defect (b), the same seed-1 Saudi
  Arabia hit-and-run seen from two sides.

## The pictures

Eight, all taken headlessly in `shot:` mode by the presentation reviewer and looked at before
filing. They show what a player sees of the military game today; none shows a proposal.

| picture | what it shows |
|---|---|
| [`mil-tall-earth.png`](mil-tall-earth.png) | China's card at 1920×2400: the **Army orders** block at the very foot, after Pioneers and Unrest, with *attack Russia (61%)*, *attack India (61%)*, *attack Indonesia (50%)*, *attack Japan (50%)*. No defender, no strength, no holder on any button. |
| [`mil-region-earth.png`](mil-region-earth.png) | The same card at 1080p: the panel ends at *Pioneers waiting: 0*. The Army orders block is below the fold. |
| [`mil-region-solar.png`](mil-region-solar.png) | The Solar System Map and the roster: Army rows read *the 1st Chinese Army, at China: strength 4 (Hold)*; four stack labels at Mars read *x1 str 0* for three Factions and *str 3* for one, and *Orbital Control: Custodians*. |
| [`mil-region-mars.png`](mil-region-mars.png) | The Mars orbit band: one line per Faction, *1 Ship(s), strength 0*, and who holds Control. |
| [`mil-stack-mars.png`](mil-stack-mars.png) | The Ship stack card: *Against Prospectors 0, Arkwrights 0 and Archivists 0 (0 in all). Attack odds (first round): 100%* and an **Attack this turn** button offered against three unarmed rivals. |
| [`mil-report-report.png`](mil-report-report.png) | The Report: a four-Faction Battle in Mars orbit sits at the foot under thirteen *have not forgiven* lines while *Coastal Engineering is complete* is the headline. Combatants are types, *Frigate, Frigate*, not names. |
| [`mil-moment-report.png`](mil-moment-report.png) | The same shot aid asked for a Battle Moment and got the plain Report: no Moment fires for a Battle in which nothing burned. |
| [`mil-threat-earth.png`](mil-threat-earth.png) | The Faction window's Holdings (*2 Ships, 1 Army*) and Under way block, the nearest thing the player has to a threat line, beside China's card with its own Influence challenger line. |

## Figures the agents cited that are worth checking before relying on them

These came from the agents' own reading of the tree and are recorded as claims, not findings:

- **55 Battles over 80 games, 46 of them against neutral Standing Armies** is read from a comment in
  `resolution.rs`, an earlier count, not from the 0.08.4 sweep, which prints no such line.
- The ground and strategy measurements are **three seeds** (1, 2, 3) and the ships measurements
  **eight** (11, 22, 33 and 1 to 5), all with seat 0 the Custodians in China. A sweep-sized count
  is the first thing the reports ask for.
- **Materials made per game** by seat, 744 / 2721 / 149 / 278 in seed 1, is the figure the
  proposals price Armies and warships against; the map's standing Ducat figure is [546, 1361, 27,
  424].
- **Zero space Battles, zero Ships destroyed by any cause, zero Armies carried** across the eight
  ships seeds and the sweep is the strongest measured claim here and the easiest to re-check.

## What the session said next

*"If you want to take this further, the natural shape is a wayfinder map for the next version, the
war version, with the four reports as its charting material. Your call; the open questions above
are the ones its tickets would carry."* No map was charted; the reports wait on the designer.
