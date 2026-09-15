# Suggestions: waking Relations up, and what diplomacy hangs off it

Brainstormed by an Opus agent on 2026-09-15 after reading the glossary, the 0.08.0 spec and the engine. Proposals only; nothing here is decided.

Two things were given to the agent as already settled by the designer and are not its ideas: that **diplomacy between Factions is coming**, and that **Blame will be factored into the Relations score**. Suggestions 2 and 3 are therefore written as option-sets on *how*, not as proposals for *whether*.

The agent's reading of the ground: `offend()` has exactly three call sites (`resolution.rs:347`, `:370`, `:735`), `settle_relations()` runs once a turn from `turn.rs:182`, the card is six numbers in `factions.toml [relations]`, and the whole score is four 4×4 arrays in `state.rs`. Almost everything below is a hook into a structure that already exists and already saves.

---

## 1. The grudge front — the computer seats read Relations when choosing where to spend

**What it is.** The AI's Influence candidate already carries a `threat` and an `opportunity` multiplier (`ai.rs`, `Candidate::score`). Add one more that reads the seat's own Relations score against the place's holder: a seat leans toward pushing on places held by a Faction it already resents, and away from one it has no quarrel with. Nothing else in the AI changes; no new category, no new order.

**Why it brings life.** Today's contests are arithmetic — the cheapest place wins — so pressure sprays evenly and no rivalry ever forms. One multiplier turns a first offence into a front, and a player watching the Report sees the same two Factions at each other's throats across three turns.

**Size.** Small: one multiplier and one weight per Faction card.

**Risks.** This is the denial multiplier ticket #50 removed wearing a new coat; pushed too hard it makes seats chase grudges instead of their own Victory Condition.

**Open design questions.** How strong (the agent recommends a very small lean — a 1.1 at −3, nothing before −2 — and measuring before widening); should it be per-Faction, so the Custodians hold grudges and the Prospectors do not; and does it also make a seat *defend* harder where it is resented, which is a different and arguably better rule.

## 2. How Blame feeds Relations (option-set)

**What it is.** Blame is already several figures: absolute Blame, `blame_share` against a fair quarter, and per-turn emissions in `climate.rs`. Four shapes:

- **(a) Share above a fair quarter, as a level.** A seat's Relations score against an offender is offset by how far that offender's share of the four Factions' Blame stands above 0.25, mirroring `blame_threshold_multiplier` exactly.
- **(b) Blame added this turn, as a rate.** The offender emitted heavily this turn, so this turn counts as an offending turn even with no Influence spent.
- **(c) Blame in Regions the viewer holds.** Only smoke over *my* territory offends me — concrete, local, and it makes the Prospectors' Strip Permit in a Custodian Region a diplomatic act.
- **(d) A separate standing modifier**, not folded into the score at all: the grid shows `−4 (−2 Blame)`.

**The agent's recommendation.** (a) as the spine, read as a **level** and recomputed each settle, so a seat that cleans up is forgiven without needing quiet turns — plus a per-Faction **resentment coefficient** on the card: Custodians ×2 or more, Archivists ×1, Arkwrights ×0.5, Prospectors ×0. That makes Blame mean four different things to four seats, which is the point of having four seats.

**Why it brings life.** It gives the climate a *social* consequence for the first time. Today Blame only raises thresholds; after this, a dirty game costs a Faction the room to deal.

**Size.** Small to medium: the figures all exist; the composition rule and four coefficients are the work.

**Risks.** A level that recomputes every turn fights the offending-turn rule, which is a memory — the two can cancel and produce a score that never moves.

**Open design questions.** Level or rate (the agent recommends level). Does the Blame part clamp separately, so a Faction can never be pushed to −10 on smoke alone? Does it compose additively, or as a multiplier on what an offending turn costs — i.e. a dirty Faction's offences hurt *more*? And does the Prospectors' zero mean they forgive Blame, or that they never noticed it.

## 3. What an Accord is (option-set)

**The word first.** `CONTEXT.md` lists *diplomacy* on Relations' own `_Avoid_` list, so the new system needs a word of its own: **Accord**, **Compact** or **Understanding**. The agent recommends **Accord**, with **Terms** for what is inside one.

**Three shapes:**

- **(a) The one-turn trade.** An Order like any other: I give you 40 Ducats, you give me 20 Materials, resolved at Resolution, cancellable until End Turn. No memory, no breaking, no upkeep. Smallest possible.
- **(b) The standing Accord.** A term that holds until one side ends it: *I spend no Influence on your places*, *my Ships do not open a Battle against yours*, *you may refuel at my stations*, *a tenth of my Research is yours*. Each term is a flag two seats carry; Relations is the price of entry and breaking one is an offence worth several offending turns.
- **(c) The Accord as a Relations gate only.** No goods change hands at all: reaching a Relations score buys *permissions* — passage through a blockaded slot, the right to found into a Colony Slot at a Body another Faction holds — and nothing is negotiated.

**The agent's recommendation.** (b), narrowed hard to **three terms at launch**: non-aggression (no Influence on my places, no opened Battle), passage (refuel at each other's stations), and a **one-turn tribute** of Ducats or Materials that (a) covers. Breaking a standing term costs a fixed, visible Relations drop — five, say, so it undoes twenty quiet turns — and is said in the Report as its own line.

**Keeping it untedious in a single-player game.** The computer seats should **propose**, not wait to be asked: an offer arrives as a **Moment** or a line at the head of the Report, with the terms and a yes/no, and the player's own offers go out from the **Faction window**, which already exists and already shows exactly the two Relations rows a player would consult. The computer's decision is one number: it accepts when what it gets exceeds what it gives by its own weights, scaled by its Relations score — which is the same arithmetic `Candidate::score` already does.

**Size.** Large, unavoidably: a new order family, a new persisted structure on the save, an acceptance routine per seat, and a window's worth of interface.

**Risks.** Single-player deals are where strategy games go to be exploited — the computer seats must never accept a trade that loses them the game, and a term nobody ever breaks is a term nobody notices.

**Open design questions.** Can a Region change hands in an Accord, or is territory never tradeable (the agent recommends never — it makes the Influence system decorative). Are Techs shareable, given a Tech already goes to everyone? Does an Accord bind for a set number of turns or until ended? And is a refused offer itself an offence?

## 4. The earned half — what fills +10

**What it is.** Version 0.08.0 reserved +1 to +10 and deliberately left it empty, because quiet alone is distance and not friendship. Fill it only with **acts**, never with time: an Accord kept for N turns, a tribute paid, a Faction spending Influence on a place a *third* Faction is pushing the viewer out of (help against a common rival), Relief paid into a Region the viewer directs, a Scrubber built in the viewer's Region.

**Why it brings life.** Without a positive half, every Accord is bought with goods alone and Relations is only a punishment track. With one, a Faction can *repair* a relationship, which is the only thing that makes the fall interesting.

**Size.** Small: the settle routine already walks every ordered pair; these are new marks alongside `offended`.

**Risks.** Positive Relations without a cost is farmable — a seat could buy goodwill with cheap Relief every turn.

**Open design questions.** Which acts count, and do they mirror the offences one for one (the agent recommends not: three or four deliberate, visible acts, each expensive, so that positive Relations is always evidence of something). Does the positive half decay back to neutral the way the negative half recovers? And can a Faction sit at +10 with a Faction it is beating to the Victory Condition, or should closing on victory drag every score down?

## 5. The Relations bite — what the score does to the board

**What it is.** One place where the score touches the rules, so a grudge is felt without an Accord existing yet. Three candidate hooks, all one line: **(a)** the score adds to the **Resistance** divisor at places the offended Faction holds — my people have learned to distrust you; **(b)** it adds to the **challenge margin** in a held place, which measurement in 0.08.0 showed is the binding figure in 190 of 200 takes; **(c)** it raises the offender's **Threshold** there, which composes naturally with Blame since Blame already does exactly this.

**The agent's recommendation.** (b), the challenge margin, because it is measurably the figure that binds and it touches only held places — which is precisely the set of places an offence can be committed against.

**Why it brings life.** It closes the loop: the first push on a rival's Region makes the second push dearer, so an attacker must either commit or leave off, and hedging is punished.

**Size.** Small: one term added to the margin lookup, plus the Region card's three-line breakdown gaining a fourth line.

**Risks.** A feedback loop — offending makes it harder, which makes the push longer, which offends more — can lock two Factions into a stalemate neither can leave.

**Open design questions.** Which hook. How steep (the agent recommends a margin of 20 + |score|/2, capped at +5). And whether the bite should be **capped well short of the scale's end**, so the −10 floor is a statement rather than a wall.

## 6. A ladder of offences

**What it is.** Today every offence costs the same one point, charged per turn: a single Influence order on a Colony equals a Battle opened against a fleet. Grade them. A short ladder: spending Influence on a held place (1), Blockading a slot at a Body they hold (1), landing an Army and beginning an Occupation (3), opening a Battle (3), taking a place from them outright (5), destroying a Scrubber or an Archive by capture (5). New offences worth adding: a **Strip Permit** in a Region bordering theirs, a **Resettle** that dumps Refugees into a state they direct, and buying out the Trading window when they are short.

**Why it brings life.** A single-point charge makes the grid a tally of contacts. A ladder makes it a record of *what was done*, and lets a player commit a small offence knowingly.

**Size.** Small to medium: the ladder is a table in `factions.toml`; the new offence sites are a handful of `offend()` calls with a weight argument.

**Risks.** Per-turn charging and a ladder interact badly — does a turn with a Battle *and* three Influence orders cost 3, or 4?

**Open design questions.** The rungs, which are pure design. Does a turn charge the **worst** offence or the **sum** (the agent recommends the worst, keeping the per-turn spirit)? And is taking a place a one-time charge or a standing offence for as long as it is held?

## 7. What you may see of a rival's view of you

**What it is.** The Faction window already shows two rows — what this seat thinks of the others, and what the others think of it — and shows them exactly. Decide whether that precision is earned. Options: **exact always** (today); **a band** (`hostile / cool / neutral`) that sharpens to a number where a Relay or an Embassy of yours stands in their territory; or **exact for your own view, banded for theirs**, sharpening only after a contact.

**Why it brings life.** Uncertainty about whether a rival has noticed you is the single cheapest source of tension in the whole angle, and it makes the Embassy and the Relay do a second job.

**Size.** Small: a band function and a condition in `relations_row` in `src/ui.rs`.

**Risks.** Fog in a single-player game that the computer does not also suffer is just a worse interface.

**Open design questions.** Whether to fog at all (the agent leans yes, but only the inward row, and a **Spectator** sees everything as they already do). What sharpens it. And whether the Report should keep naming the exact figure when a score falls, which would defeat the fog.

## 8. The scar — time does not heal everything

**What it is.** Recovery today is uniform: +1 per four quiet turns, back to 0, forever. Give the pair a memory. Either a **floor that ratchets** — every third offence lowers the best a pair can recover to, so a seat that crossed you eight times never returns to neutral — or a **slowing recovery**, where the quiet turns needed grows with how many offences the pair has on record.

**Why it brings life.** A thirty-six turn game has room for a whole relationship to go bad and stay bad. Full healing means the last five turns look like the first five.

**Size.** Small: one counter per ordered pair in `Relations`, and two lines in `settle_relations`.

**Risks.** By turn 30 every pair is floored at the worst and the score stops carrying information.

**Open design questions.** Ratchet or slow (the agent recommends the ratchet — it is legible on the grid, and the Faction window can print the floor beside the score). How many offences per step. And whether an **Accord kept**, or an act from suggestion 4, can lift the floor — which would make peace-making the only thing in the game that undoes a scar.

---

**The agent's own "if you only take one":** how Blame feeds Relations (2), because the per-Faction resentment coefficient is what makes four seats feel like four different minds rather than one mind with four Victory Conditions — the Custodians resenting smoke while the Prospectors do not notice it is a whole character in one number. Pair it with the grudge front (1) if a second is affordable, since a score nothing acts on is still a score nothing acts on, and one multiplier in `ai.rs` is the cheapest teeth on this list. Everything else — the Accord above all — is better designed once a version of Relations that actually moves has been watched.
