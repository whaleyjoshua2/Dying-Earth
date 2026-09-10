# Suggestions: player experience, pacing, drama and the feel of a session

Brainstormed by an Opus agent on 2026-09-09 after reading the spec, the glossary, the playtest note, `src/ui.rs` and the engine's turn and resolution code. Proposals only; nothing here is decided.

---

## 1. The Report as a dated dispatch

**Small–Medium.** The Report phase already collects everything worth telling — `report.lines`, `report.battles`, last turn's Event — but shows it as one flat list, which is why the most dramatic turn of the game reads the same as the dullest. Give the Turn a real date (Turn 1 = a month the designer picks, running 24 months) and open the Report as a dated bulletin: one **headline** chosen by a fixed severity order (Colony founded > place changed hands > Sea Level threshold > Battle with a unit destroyed > Occupation begun or transferred > Event drawn > builds completed), then the remaining lines grouped under four headings — *In space*, *On Earth*, *The climate*, *Your works*. Every line is clickable and selects its place and switches to its view, exactly as the roster rows already do. Nothing in the engine changes; each `report.lines` push gains a kind tag and a `Place`.

**Why:** it turns the phase that opens every turn from a receipt into a story, and makes the player's first act of the turn a tour of what matters rather than a scroll and a dismiss.

**Risk:** none to the rules. Dating the turns is a fiction commitment the designer should make deliberately (a start year fixes how near-future the setting is).

---

## 2. The rival has a name and a temperament

**Medium.** The AI is already a pure weight machine (`ai.toml` base weights, victory-gap, denial, threat and opportunity multipliers), so a personality is a second table on top of it: a named opposing leader drawn per seed, with three or four weight deltas and one plain-English creed shown on the Faction screen at New Game. A Prospector "the Admiral" doubles Frigate, Battleship and Attack weights and halves Research Lab; "the Speculator" raises Bank, Trade Post and the Ducat buy and prefers Influence over Armies; a Custodian "the Gardener" pushes Restoration and Green Consensus, "the Missionary" pushes Embassy, Relay and Influence at the cost of producers. The creed is shown once and never again — the player must read the board to confirm it.

**Why:** an opponent with a stated temperament is something you can plan against and be surprised by, and it converts twenty replays of the same seed-shuffled game into twenty different opponents.

**Risk:** it will move the balance the 0.02–0.04 sweeps established, and version 0.04's finding that the Custodians already win only two games in twenty means an aggressive Prospector temperament could push that to zero. Every temperament needs its own twenty-seed `simulate` run, and the deltas want to be small.

---

## 3. The Intelligence panel: what the rival did last turn

**Medium.** A panel (top-bar button, beside Victory) that reports only what the rival did *last* turn, never what it plans: which places its Standing rose on and by how much, what it built and where, what is in transit with destination and turns remaining, its Extraction Total or Stabilization run against its bar, and a single derived sentence naming its heaviest category of spend ("Last month the Prospectors put everything into Africa and Asia"). All of this is already visible in principle — the AI plays with the same visibility — but it is scattered across nine state cards and a Solar System Map, so nobody reads it.

**Why:** it makes the AI legible enough to be an antagonist rather than weather, and it creates the specific tension of watching a Standing climb toward your challenge margin two turns before it flips.

**Risk:** report *last turn's actual spend*, not this turn's scored list, or the player can counter-spend perfectly and the Influence game collapses. Keep the simulate-mode scored dump (16.5) out of it.

---

## 4. Moments: four things the game stops for

**Small–Medium.** A short modal, one sentence and one number, that interrupts the Report for exactly four kinds of thing and nothing else: the founding of a Colony (with its real name and Body — "Tycho on the Moon is founded, the first Colony off Earth"), a Sea Level threshold crossing (with the build slots each Nation State just lost, permanently), a Battle in which a unit was destroyed or Orbital Control changed hands, and a Nation State or Colony changing controller. Cap at two per turn, highest severity first, and let the rest fall through to the Report list. A single "stop for" checklist in a settings corner lets a player who has seen it twenty times turn any of them off.

**Why:** a 24-turn session needs peaks, and right now the founding of the first Colony — the thing both Victory Conditions are built on — is one line among fifteen.

**Risk:** modal fatigue, especially in a late-game turn with three Battles. The cap and the opt-out are load-bearing, not polish.

---

## 5. The Chronicle, and the end of the game

**Medium.** The game-over screen is currently four lines and a seed. Replace it with a chronicle: one headline per Turn drawn from the Report's severity ranking (the same function as suggestion 1), four small graphs across the 24 turns (Temperature against the Collapse Line, CO2 Stock against the Natural Sink, Extraction Total against 500, Colonists off Earth against 12), a superlatives block (first Colony and its turn, largest Battle, hottest turn, most contested Nation State, how many Events the Event Deck still held), the seed, a "Replay this seed" button, and "Copy chronicle" which puts the whole thing on the clipboard as plain text.

**Why:** with no save, the chronicle is the only thing a player keeps from a session, and it is what they paste into the playtest reply the `PLAYTEST.txt` asks for.

**Risk:** none mechanically. It needs per-turn history kept in `Game` (four numbers and one string a turn is nothing), which must survive the End phase that currently resets `self.report`.

---

## 6. Board lenses

**Small–Medium.** Number keys 1–4 recolour whichever map is up, instead of the single controller tint the Earth Map has now. **Control**: as today. **Standing**: every Nation State, Colony and Space Station shaded by how close it is to flipping, with a two-bar gauge showing both Factions' Standings against the threshold and the challenge margin. **Emissions**: each Nation State shaded by its Emissions this turn, so the Climate Panel's numbers have a picture. **Yield**: every Colony Slot and Orbital Slot shaded by what it would produce for you, so choosing between Hellas Planitia and Stickney is a glance rather than four panel clicks. The lens persists across the Tab swap between the Solar System Map and a Body Surface Map.

**Why:** a board game player expects to read the board's state in one look; at present the Standings that decide most of the game live only inside individual cards.

**Risk:** none. The Standing lens will make it obvious how much ping-pong is still in the Influence game (0.03's open question 1, partly addressed by 0.04's margin) — that is a feature, but it will draw complaints.

---

## 7. Prices that move, and a lot limit

**Medium.** The Trading window is a fixed price table with no caps, which version 0.04 already flags as an exploit (open question 3). Make each line's price a base times a scarcity factor recomputed each Income from the board: Materials rise as the two Factions' combined Mine and Factory output falls, Fuel rises with launches made last turn, Energy falls when producers were shut down by the shortfall rule. Show last three turns' price and an up or down arrow beside each line, and cap the lot bought per Turn at something proportional to the Ducats the Faction's own Nation States and Banks earned that turn, so the window is an outlet for an economy rather than a substitute for one.

**Why:** it makes Ducats feel like money in a world instead of a second currency with a fixed exchange rate, and it creates the genuinely interesting decision of buying Fuel before a launch window rather than during one.

**Risk:** this is a balance change, not a presentation change, and the Ducat rates (2 for 1 on Influence, 20 per Restoration step, 10 per repair point) were tuned against fixed prices. The drift range wants to be narrow — the designer should set the floor and ceiling, and whether the cap is per-line or across the window.

---

## 8. Scenarios and a difficulty dial

**Medium.** The game is already entirely TOML-driven, so a scenario is a start-position file: which Nation States each Faction holds, what stands in them, the Stockpile, the CO2 Stock, the turn number, the turn count and the bars. Ship four beside the default: **Late Start** (open on turn 12's board at +1.9 °C, twelve turns to fix it), **Rush** (twelve turns, halved bars — a lunch-break game), **Cold War** (each Faction starts with three states and a Shipyard, so Earth is settled and the fight is off-world from turn 1), and **The Long Fall** (36 turns, the full arc). Difficulty is three named settings that scale the AI's Allotment and output multipliers by 0.85 / 1.0 / 1.2 and are stated in plain words on the New Game screen, not hidden.

**Why:** it fixes the two worst pacing complaints at once — the slow opening (0.04 moved the first Colony from turn 7 to turn 10) and the absence of anything to do differently on a second play.

**Risk:** every scenario is a balance surface the anchors of 19.3 do not cover. Ship them as clearly labelled and unbalanced, or run twenty seeds each.

---

## 9. Opening Orders for the first three turns

**Small.** Section 20.3 sets "no tutorial beyond the first Report" as a default, and the playtest note's first question is "the moment you were not sure what to do next" — the two are related. For turns 1, 2 and 3 only, the Report carries a boxed *Suggested next step* with one or two named actions, each with a button that queues the order for you and each cancellable like any other order until End Turn: turn 1, "Build a Shipyard on the ISS — Ships are built only at a Shipyard" and "Spend your whole Allotment on one neighbouring Nation State"; turn 2, "Build a Colony Ship" and "Raise Industry Level"; turn 3, "Lift Colonists from your Launch Site" and "Set the transit". The box never appears again and says so.

**Why:** the opening of this game is genuinely obscure — a bare Space Station, one Nation State, and a chain of five prerequisites before the first Colonist leaves Earth — and three turns of scaffolding is the difference between a playtester reaching the midgame and quitting on turn 4.

**Risk:** it overrides a stated default, so it is the designer's call. Keep it advisory and cancellable; the moment it queues something the player cannot undo it stops being a tutorial and starts being a rail.

---

## 10. The Research Lead race, made a moment

**Small.** The split between the two Factions' contributions to the current Tech already exists and is already rendered as one line of text. Put it in the top bar as a live two-colour bar beside the Tech under research, so the race is visible every turn and building a Research Lab in a high Education Level state visibly moves it. When a Tech completes, stop the Report for it: a modal naming the Research Lead and the margin, the Tech Tree with the newly unlocked boxes lit, and either the player's Pick buttons or one line saying what the AI took and, in its leader's voice (suggestion 2), why.

**Why:** the Research Lead is the one contested resource in the game with no board presence at all, and turning the pick into a moment gives the player a reason to care about a Lab in Australia.

**Risk:** none. It should be one of the "moments" in suggestion 4's severity list rather than a fifth independent modal.

---

## 11. The deck on the table

**Small.** The Event Deck is 28 cards, shuffled with the seed and **never reshuffled**, with about fifteen drawn over 24 turns — which means the deck depleting is already a real arc that the player currently cannot see. Draw it as a physical stack somewhere permanent (the Climate Panel is the natural home): cards remaining, the count by kind still in it ("13 left: 5 Climate, 4 Failure, 2 Solar, 2 Discovery"), this turn's Draw Chance as a percentage, and a discard row of the cards already seen. Never the order, only the composition.

**Why:** it lets a board game player do the thing board game players do — count what is left and take a risk on it — and it makes the Draw Chance's rise with Temperature something felt rather than read.

**Risk:** knowing that no Storm Surge remains removes some late-game dread. It also makes visible that the deck can empty before turn 24, which the designer may want to answer (stop drawing, or reshuffle the discard) — currently the rule is silent past the last card.

---

## Also considered, and why they rank lower

- **In-flight hard burn** (spend half a transit's Fuel again to arrive a turn early) would give in-flight Ships an order every turn instead of the roster's "no order", and would fix real dead time on the 4-turn Mars run — but it bends 9.1's "a Ship in flight cannot fight, be fought, or turn back", which is load-bearing for how simple transits are. Worth asking about; not worth taking without the designer's word.
- **Fog of war on the rival's Standings** would sharpen suggestion 3 enormously, but the spec's AI plays with the same visibility as the player, and asymmetric information is a much larger change than it looks.
- **Save and load** is the one thing a 24-turn session most obviously wants (section 20.1 defers it), but it is not a mechanic and it is not a small job; it belongs on its own ticket.
