# 2026-09-15: bringing life to the Faction system

Six Opus agents were each given one angle on the Faction system and asked for six to nine concrete suggestions, ranked best first, with a size, a risk and the open design questions on each. They read the glossary, the 0.08.0 and 0.08.1 specs and the engine. Nothing here is decided; the lists exist so the designer can pick.

Two things were handed to every agent as **already settled** and are therefore not proposed anywhere as new: **diplomacy between Factions is on the roadmap**, and **Blame will be factored into the Relations score**. Where the files discuss either, they are proposing a *shape* for a decision already taken.

| File | Angle | Count |
| --- | --- | --- |
| [01-computer-seats-as-characters.md](01-computer-seats-as-characters.md) | The computer seats as players with personalities | 8 |
| [02-relations-and-accords.md](02-relations-and-accords.md) | Waking Relations up, and what diplomacy hangs off it | 8 |
| [03-asymmetry.md](03-asymmetry.md) | The four Factions playing genuinely different games | 8 |
| [04-voice-and-presentation.md](04-voice-and-presentation.md) | A Faction as a character, not a colour | 8 |
| [05-pressure-without-fleets.md](05-pressure-without-fleets.md) | The ways Factions collide that are not armies | 8 |
| [06-reading-the-rivals.md](06-reading-the-rivals.md) | What the player knows of a rival, when, and how they feel it | 8 |

## Ideas that came up more than once

Agreement between agents working from different angles is some evidence an idea is load-bearing.

- **A Moment that belongs to a rival**, fired when its Victory progress crosses a high bar — Voice #1 and Reading #1. Both agents made it their single "if you only take one", from opposite directions: one because a Faction that can frighten you is a character, the other because nothing in the game answers "they are about to win" without being asked.
- **The computer seats reading Relations** — Characters #4 and Relations #1. Both independently identified it as ticket #50's denial multiplier returning, and both proposed the same guard: a very small lean, budget-neutral so a grudge redirects spending rather than escalating it, measured rather than assumed.
- **Trading prices that move with what the table bought** — Pressure #3, and previously Space #9 and Experience #7 in the 2026-09-09 suggestions. Third time it has been raised.
- **The Arkwrights are the thin Faction** — the framing finding of Asymmetry, and reached again from the AI side in Characters (they are the seat with no distinct opening) and from the endgame in Asymmetry #5 (their Victory Condition pays nothing until complete, and on the current board that means never).

## Two gates rather than features

Both are rules about what the other suggestions may spend, and both are cheaper to settle before building than after.

- **The voice budget** (Voice #5): at most one line of voice reaches the player unasked per turn, across all four Factions. It caps Voice #1, #2 and #6.
- **Delayed sight** (Reading #8): whether a rival's Victory progress reads live or as of the last Report. It settles whether this game wants imperfect information at all, which Relations #7, Reading #4 and Reading #6 all depend on.

## Figures the agents cited that are worth checking before relying on them

These came from the agents' own reading of the repo and are recorded here as claims, not as findings:

- A win table of **42 / 11 / 7 / 6** across 80 games (Custodians / Prospectors / Arkwrights / Archivists). Several balance suggestions are justified by it.
- **No Faction has ever settled beyond the Moon** in those games — cited as the reason the Arkwrights' Diaspora is unreachable and as the case for the Ark.
- The Tech Tree completes by **turn 16 of 36**.
- `blame_credit` is computed in `state.rs` and drawn in the interface, but **no rule reads it**.
- The challenge margin, not the threshold, was the binding figure in **190 of 200** takes measured in 0.08.0.
