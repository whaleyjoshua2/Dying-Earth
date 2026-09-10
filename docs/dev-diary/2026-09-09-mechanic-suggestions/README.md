# 2026-09-09: mechanic and feature suggestions

Six Opus agents were each given one angle on the game and asked for eight to twelve concrete mechanics or features to add, ranked best first, with a size and a risk on each. They read the spec, the amendments, the glossary and the playtest note. Nothing here is decided; the lists exist so the designer can pick.

| File | Angle | Count |
| --- | --- | --- |
| [01-climate-and-earth.md](01-climate-and-earth.md) | The Climate Model and Earth-side play | 12 |
| [02-space-economy.md](02-space-economy.md) | The space economy, colonization and logistics | 12 |
| [03-combat-and-military.md](03-combat-and-military.md) | Conflict, combat and military play | 12 |
| [04-factions-and-victory.md](04-factions-and-victory.md) | Factions, asymmetry, victory and the endgame | 12 |
| [05-tech-and-events.md](05-tech-and-events.md) | Research, the Tech Tree and the Event Deck | 12 |
| [06-player-experience.md](06-player-experience.md) | Player experience, pacing and drama | 11 |

## Ideas that came up more than once

Several agents arrived at the same thing from different directions, which is some evidence the idea is load-bearing:

- **Blame for Emissions raising Influence thresholds** — Climate #5 and Factions #2.
- **The Event Deck shown as a countable stack** — Tech #7 and Experience #11.
- **Trading prices that move** — Space #9 (moves with what you buy) and Experience #7 (moves with board scarcity, plus a lot cap).
- **Each Colony Slot having its own character** — Space #1 (fixed multipliers per named site) and Tech #5 (face-down Discoveries, surveyable).
- **A Trade Post paying for a network of Bodies** — Space #10 and the Syndics' Freight Lanes in Factions #1.
- **Something to see when a Tech completes or the Lead is decided** — Tech #4, Tech #10, Factions #11, Experience #10.

## Standing findings the agents kept citing

- Population Emissions alone exceed the Natural Sink before anything is built, so no Faction ever meets its Victory Condition in AI-versus-AI play (0.02 §8.1, still open in 0.04).
- The AI never raises an Army, never builds a Carrier, a Bank, a Trade Post, a Relay or a second station, and colonizes only Mars. Any new piece needs an `ai.toml` weight or it will sit unbuilt.
- Research has no decision in it; the Trading window has no caps.
