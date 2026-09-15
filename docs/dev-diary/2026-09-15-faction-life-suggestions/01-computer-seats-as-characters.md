# Suggestions: the computer seats as players with personalities

Brainstormed by an Opus agent on 2026-09-15 after reading the glossary, the 0.08.0 and 0.08.1 specs and `engine/src/ai.rs`. Proposals only; nothing here is decided.

The agent's reading of where the seats stand today: one shared greedy scorer, where a seat's whole character is `[weights.*]`, `[pace.*]` and `[tech_picks.*]` in `assets/data/ai.toml` plus about six `kind ==` special cases (Scrubber, Leapfrog, Strip Permit, Carrier, and the Prospectors' looser attack terms). Every other seat fights on identical terms, covets identical Regions, and reacts to being attacked identically.

---

## 1. Covetousness — each Faction reads a Region's worth its own way

**What it is.** `ai_orders` ranks every Influence target by one formula for all four seats: the Region's Influence value + Industry Level, +2 if adjacent, ×0.3 if held (`held_state_weight`). Replace that single formula with one per Faction. Prospectors read Resource Lean and Industry Level. Arkwrights read population — Regions are Emigrant wells — and a working Launch Site. Archivists read Education Level (their Labs, and low Resistance elsewhere). Custodians read Baseline Emissions × population, what a Leapfrog would bite. Add a **persistence clause**: a seat keeps last turn's top target while it stays within, say, 20% of the new best, so spending stops wandering.

**Why it brings life.** The player learns the map four different ways and can say "Australia is a Prospector Region" before anyone has spent a point there.

**Size.** Small — one function plus a per-Faction table in `ai.toml`.

**Risks.** A narrow reading could leave a seat bidding for Regions it can never afford; the fallback must stay the current formula.

**Open design questions.** Which figure each Faction reads (the agent recommends the four above). How hard the persistence clause bites — a hard lock, or the soft 20% band (recommended: the soft band; a hard lock makes a seat exploitable).

## 2. The Intent line — a seat says what it is saving for

**What it is.** The scorer already computes this and throws it away: the `reserve` / `ducat_reserve` / `fuel_held_for` lines log "wait 41.2 build a Colony Ship (affordable within four turns)". Surface the held-for note as a second sentence in the Report's rival paragraph, which already renders deeds from `report.toml`'s `[rival]` table: "The Prospectors are holding Materials for a Factory in Australia."

**Why it brings life.** The seat stops being a list of deeds and becomes something with a plan you can pre-empt.

**Size.** Small — one field on the AI's return, one template line.

**Risks.** Perfect honesty every turn makes rivals trivially countered.

**Open design questions.** Does every Faction telegraph, or is reticence itself characterisation (the agent recommends: Prospectors and Arkwrights announce, Custodians announce only their Leapfrogs, Archivists say nothing — and make that a stated trait, not a bug)? Is it always true, or may a seat name a decoy?

## 3. The opening book — each Faction's first four turns are its own

**What it is.** Turns 1–4 draw from a short per-Faction list of preferred first orders that outweigh the general scorer, then it hands back. Prospectors: raise Industry Level, Factory, Strip Permit. Arkwrights: Launch Site, then their Spaceport, then a station. Archivists: Research Lab, then the chain out of Earth orbit. Custodians: Research Lab, then the first Scrubber. Two or three books per Faction, picked by seed.

**Why it brings life.** Openings are how board-game opponents become familiar; it also fixes by construction the shape the 0.08.1 sweep exposed, where the Archivists needed `archive_needs_a_place = 5.0` bolted on to make them leave.

**Size.** Small-medium — a table in `ai.toml` and a pre-pass in `ai_orders`.

**Risks.** A fixed opening is learned once and then dull, and a bad book locks a seat into a losing line for four turns.

**Open design questions.** How many books each Faction gets, and whether the book is abandoned when the board contradicts it (recommended: abandon it if the seat loses its start Region).

## 4. Grudge — the computer seats finally read Relations

**What it is.** Relations exist, are scored per ordered pair, and nothing reads them (0.08.0 section 7 says so deliberately). Let a seat read only its own row: where Relations with a Faction sit at −4 or worse, that Faction's held Regions are ranked with a raised `held_state_weight` **and the seat's other targets are lowered to match**, so total Influence spent does not rise — the grudge redirects, it never escalates. Same shape for the attack odds bar against that Faction alone.

**Why it brings life.** A rival that remembers who hit it first is the single largest step from arithmetic to somebody.

**Size.** Medium — small in code, but it moves the sweep and wants re-measuring against 42 / 11 / 7 / 6.

**Risks.** This is ticket #50's denial multiplier coming back through the window; budget-neutral redirection is the guard, and it must be measured, not assumed.

**Open design questions.** At what Relations figure the grudge opens; whether every Faction holds one or only some — the agent recommends that the Prospectors and Arkwrights carry grudges while the Custodians and Archivists do not, and that the asymmetry is characterisation. Whether Blame feeding Relations should also feed the grudge, so a dirty seat is *disliked* rather than merely harder to sway.

## 5. Risk appetite — one number per Faction, not one for the table

**What it is.** `attack_odds = 0.6` and `evade_damage_fraction = 0.6667` are shared by all four. Make them per Faction, and let the same dial govern the Materials reserve horizon and whether a Colony Ship takes Crowding. Prospectors gamble at 0.45 and crowd freely; Archivists refuse below 0.8 and never crowd; Arkwrights crowd every lift, because people are the point of them; Custodians sit at 0.7 and evade early.

**Why it brings life.** Two seats in the same position make visibly different calls, which is exactly what a player reads as character.

**Size.** Small — the tables are already per-Faction shaped.

**Risks.** A low bar plus the Prospectors' existing licence to attack anything could make them a runaway military problem.

**Open design questions.** The four figures, and whether Crowding should follow the same dial or be its own trait (recommended: its own, since it is a Colonist-lives decision and reads differently).

## 6. Creed — the refusals a Faction will not trade away

**What it is.** Hard bans rather than low weights, so the player can bank on them. Custodians will never take a Strip Permit and will never let a controlled Region's Emissions rise while a Scrubber slot is free. Archivists will never Decommission a Research building and never Mothball a Lab, even when Energy is tight. Arkwrights will never leave a mustered Emigrant behind while a Ship has room. Prospectors will never pay for Relief when the Ducats could buy Materials. Each costs the seat something real in some turn.

**Why it brings life.** A Faction that will visibly hurt itself rather than act out of character is a character; an optimiser is a formula.

**Size.** Medium — a refusal pass over the candidate list, plus sweep work to see what each ban costs.

**Risks.** A creed that bites every turn is a permanent handicap, and the 0.08.1 table is finally even.

**Open design questions.** Which refusals, and whether a seat about to lose everything may break its creed (the agent recommends: yes for the Prospectors and the Arkwrights, never for the Custodians and the Archivists — and say so on the Faction card).

## 7. Desperation and the lead — distinct reactions to the same gap

**What it is.** `victory_gap` already measures how far behind a seat is, but every Faction answers it the same way: ×3 on the categories that advance its own bar. Give each a distinct move at wide gap. Prospectors chain Strip Permits and sell down the Stockpile. Arkwrights crowd every Colony Ship and take worse Bodies. Archivists stop paying the shared Tech altogether. Custodians mothball Earth industry wholesale and lean on Production Moved. And a matching **leading** behaviour past, say, 80% of the bar: hold what you have, buy Influence, stop expanding.

**Why it brings life.** A rival visibly panicking or visibly closing the door is the most dramatic thing a turn can contain.

**Size.** Medium — four behaviours plus a lead branch.

**Risks.** Desperation moves are all-in and could hand a seat the game or lose it outright; leading behaviour risks a seat stalling short of its own bar.

**Open design questions.** The gap at which each opens, whether the lead behaviour exists at all (the agent recommends yes — a leader that keeps sprinting is unbeatable and unreadable), and whether a desperation move is announced in the Report.

## 8. Temperament — the character setting the game does not have

**What it is.** `first-playable.md` line 470 says "One difficulty." Give each seat a temperament chosen on the setup screen — **Cautious / True to type / Ruthless** — that scales the risk appetite (5), the grudge threshold (4) and how hard the creed (6) binds. Not a cheat multiplier: the seat still plays by the same rules, costs and Allotment.

**Why it brings life.** It makes personality a thing the player can turn up, and it gives the project a difficulty axis that is characterful rather than numeric.

**Size.** Medium — small in the engine, but it is a setup-screen control, a save field and a Faction window line.

**Risks.** Three temperaments × four Factions is twelve seatings to sweep, which is a real measuring cost.

**Open design questions.** Whether temperament is per seat or one setting for the table; whether the player sees a rival's temperament or must infer it — the agent recommends per seat, shown on the setup screen, because a hidden dial reads as inconsistency rather than character.

---

**The agent's own "if you only take one":** Covetousness (1). It is the smallest change that makes all four seats behave differently every single turn, on the one board the player watches hardest — the Earth Map's Standing — and it costs one function and one table. Intent (2) is the cheapest thing on the list and pairs with it: the player then sees a rival want a Region and say so. Everything else is better attempted after the 0.08.1 table of 42 / 11 / 7 / 6 has been re-measured with those two in.
