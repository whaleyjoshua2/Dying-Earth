# Suggestions: the Climate Model and Earth-side play

Brainstormed by an Opus agent on 2026-09-09 after reading the spec, the glossary and the playtest note. Proposals only; nothing here is decided.

A note on why several of these cluster where they do. In the shipped numbers, **Population Emissions alone are about 8.0 ppm a turn** (0.1 per hundred million across ~80 units of population) against a **Natural Sink of 6.0**. Before a single Factory is built, the world is over the Sink. That is why "no Faction ever meets its Victory Condition" (0.02 §8.1, still open in 0.04 §9) and why the Custodians have no real Earth-side game: there is currently **no order in the entire spec that lowers Emissions**, only Techs and one-turn Restoration. Several suggestions below exist to give the Custodians verbs, and to make the Prospectors' pollution cost them something other than the shared Collapse Line.

---

## 1. Mothball and Decommission a Facility

**Small**

A Facility on a Nation State card gets two new orders beside its build buttons. **Mothball** takes one turn, costs nothing, and puts the Facility offline permanently until restarted: it produces nothing, pays no Energy upkeep, and **emits nothing** — it still holds its build slot. **Decommission** takes one turn, refunds half its Materials, frees the slot, and removes it from the board. Restarting a mothballed Facility takes one turn and 5 Materials. This plugs straight into Emissions by source (§11.2), the Energy shortfall rule (§7.2, a mothballed producer is simply not in the list), the Stabilization run and the Extraction Total (a mothballed Factory adds nothing to either).

**Why it makes the game better:** it turns "I inherited a dirty state" from a fact into a decision — every mothballed Refinery is 1.5 ppm bought with 3 Fuel a turn forgone, and the Prospectors get a use for it too when Energy runs short.

**Risk:** the Custodians could mothball their way to Stabilization trivially. Guard it by making the emissions saving real but the output loss painful, and by pairing it with Unrest (#3) so gutting a populous state's industry costs you its loyalty. Also conflicts mildly with the current implication that Industry Level "never falls" (§8.2) — keep that true; mothballing removes Facilities' emissions, not the state's Baseline Emissions × Industry Level line.

---

## 2. Per-person Emissions follow Industry Level

**Small**

Replace the flat 0.1 ppm per hundred million (§11.2, Population line) with **0.04 + 0.03 × the state's Industry Level per hundred million** — rich states emit per head, poor ones barely. A Custodian gets a matching order, **Leapfrog**: spend 40 Ducats and one turn on a controlled state to reduce that state's per-person coefficient by one Industry Level's worth, permanently, without lowering its Industry Level. Green Consensus keeps halving the whole line. The numbers should be tuned so that the eight states at their starting Industry Levels produce roughly what they do today, so the pacing sweep of 0.02 §1 holds.

**Why it makes the game better:** it puts the real-world dilemma on the board — Asia's 43.5 population is the game's richest prize and its biggest climate risk, and raising its Industry Level for the Ducats and build slots is now a decision with a visible price rather than a free upgrade.

**Risk:** it makes the Prospectors' Cheap Industry considerably more dangerous for everybody, which is thematically right but will move the Collapse turn; expect to re-run the sweep in `engine/examples/sweep.rs`. It also makes Africa (population 14.0, Industry 1) newly attractive to hold *undeveloped*, which is an uncomfortable and interesting position to be in.

---

## 3. Unrest, a Nation State's own agency

**Medium**

Every Nation State carries an **Unrest** figure, 0 to 10, shown on its card. It rises each turn by the Temperature damage that state took (population loss from heat, a lost build slot to Sea Level, a Heatwave or Wildfire card landing there) and by any Facility mothballed or decommissioned there; it falls by 1 for every 10 Ducats its controller spends on it in Orders, and by 1 whenever the controller's Restoration runs. At Unrest 4 the state's Standing Army stops replenishing; at 7 its Facilities produce at half; at 10 it **throws off its controller** and becomes neutral at once, both Factions' Standings intact, and its Armies become the state's own. The existing Unrest Event card (0.02 §4) becomes "+3 Unrest" instead of its current effect.

**Why it makes the game better:** Nation States stop being tiles and start being places with a breaking point, and it makes the Prospector who holds the whole warming globe pay for it in the one currency they cannot mine.

**Risk:** it is a second control-loss mechanism alongside Influence and Occupation, and could make holding anything feel unfair; keep the Ducat cure cheap enough that a solvent Faction always has the option, so losing a state to Unrest is always a choice not to pay.

---

## 4. Named Tipping Points in the Climate Model

**Small**

Add a table of **Tipping Points** to `climate.toml`, each a Temperature and a permanent effect that fires once, in the Climate phase, exactly as Sea Level thresholds do (§11.4): **Permafrost Thaw at +1.7 °C** adds a permanent +1.5 ppm a turn to Emissions; **Amazon Dieback at +2.1 °C** cuts the Natural Sink from 6.0 to 4.5 and doubles South America's Baseline Emissions; **Ice Albedo at +2.5 °C** halves the effect of Restoration and adds +1.0 ppm. Each is drawn on the Climate Panel as a marked notch on the Temperature bar ahead of the current reading, with the projection line (§11.5) saying which ones the current net emissions will cross and on what turn. The Permafrost Thaw Event card (0.02 §4) becomes the harbinger rather than the thing itself.

**Why it makes the game better:** it converts the Temperature from a linear ramp into a slope that steepens, so that stopping *now* is worth measurably more than stopping in three turns — the single best cure for a climate model that feels like a timer.

**Risk:** it directly contradicts the glossary's Collapse Line entry ("Every other effect of Temperature is continuous; this is the only line"), though Sea Level thresholds already break that promise; the glossary line needs rewording either way. It also risks making a mid-game Custodian position unrecoverable — mitigate by making every notch visible from turn one, so crossing one is a forecast that came true, never a surprise.

---

## 5. Blame, and what the neutral world thinks of you

**Medium**

Each Faction accumulates **Blame**: the cumulative Emissions attributable to sources it controlled, counted at the Climate phase exactly as the Climate Panel already attributes them by source and by controlling Faction's multiplier (§11.2). Blame is shown on the Victory panel beside both Factions' progress. Its effect is on Influence: a Faction's **Influence threshold on any Nation State it does not control is raised by 1 for every 20 ppm of Blame it carries**, and its Standing on such places decays 3 a turn instead of 2 once its Blame exceeds the rival's by 100. It never affects Colonies or Space Stations — off Earth, nobody is watching.

**Why it makes the game better:** it is the sharpest available fix for the Custodian/Prospector asymmetry on Earth, because it makes the Prospectors' whole strategy quietly close the diplomatic door behind them, and gives the Custodian's ×1.3 Allotment a second, thematically-earned edge instead of just a bigger number.

**Risk:** it compounds with the 0.02 finding that "the Custodian AI takes every Nation State by Influence"; it may need the Custodian Allotment multiplier lowered to 1.15 in the same change. Attribution is also fiddly for a state that changed hands mid-game — the simplest rule is that whoever controlled it at the Climate phase wears it.

---

## 6. Neutral states develop themselves

**Small**

A neutral Nation State is not inert. Every **four turns**, a neutral state whose Industry Level is below 4 raises it by one on its own, for free, at a 1.0 Emissions multiplier — and, per 0.02 §3, a Facility nobody directs stands idle, so add that a neutral state that raises its Industry Level also brings **one idle start Facility online** at the same multiplier. The Report names it: "Africa raised its Industry Level to 2." A state under Occupation, or controlled, does not do this.

**Why it makes the game better:** it removes the abstention strategy — the world warms whether or not the player builds anything — and it gives the Custodians a genuine, uncomfortable reason to conquer Earth: taking a state is the only way to stop it developing.

**Risk:** it accelerates the pace and will move the Collapse turn earlier; the Sink or `ppm_step` will need re-sweeping. It also makes the early-game map feel like it is running away from a slow player, which may need the interval stretched to six turns.

---

## 7. Sea Walls, and adaptation as a real alternative

**Small**

A new Facility, the **Sea Wall**: 35 Materials, two turns, 1 Energy upkeep, no Emissions, occupies one build slot. While it stands in a state, that state's next Sea Level threshold takes **no build slots** — the wall absorbs it and is itself destroyed in the process. Storm Surge (§13.2), which applies a threshold early, consumes it the same way. Only one may stand in a state at a time.

**Why it makes the game better:** it makes Coastal Exposure a stat you can spend against instead of only suffer, and forces the honest strategic question the real subject poses — do I fix the cause, or protect my own coast and let the Temperature run?

**Risk:** it costs a build slot to protect build slots, so it must be priced so that it is right for a high-exposure, high-Industry state (Asia, Coastal Exposure 2) and wrong elsewhere, or nobody builds it. Sea Level thresholds are currently a hard, once-only global sweep — the "fires once per state" bookkeeping already exists for Storm Surge, so this fits, but check the formula test that pins §11.4.

---

## 8. Refugees along the adjacency graph

**Medium**

When a state loses build slots to Sea Level, or its population falls in a Climate phase, **half of the lost population moves** rather than vanishing: it is distributed along the existing continent adjacency (§4.2, 0.02 §2) to neighbouring states in proportion to their Industry Level. Each receiving state gains that population — and so its Population Emissions, its Research population factor, and **+1 Unrest** (#3). A controller may pre-empt this in Orders with **Resettle**: 20 Ducats moves an incoming refugee flow into any state that Faction controls instead, gaining +5 Standing there.

**Why it makes the game better:** it makes your neighbour's Coastal Exposure your problem, and turns the emptying of a drowning continent into a hand you can play rather than a number ticking down.

**Risk:** population is currently the input to Emissions, so moving it between states with different Industry Levels (under #2) changes the global total — that is realistic but means refugee flows nudge the climate, which may read as noise. It is also the suggestion here most likely to need its own panel to be legible at all.

---

## 9. Committed Warming, and the turn it is too late

**Small**

The Climate Panel (§11.5) gains two lines above its projection. **Committed Warming**: the Temperature the world reaches if net Emissions fell to zero this turn and stayed there — that is, the target Temperature implied by the CO2 Stock as it stands, which the lag has not yet delivered. And **the Last Turn**: the latest turn on which cutting net Emissions to zero still keeps the Temperature under the Collapse Line, computed by running the existing model forward with net set to zero from each future turn in turn. Both are one line of text over arithmetic the engine already does.

**Why it makes the game better:** it is the cheapest thing on this list and it does the most for the brief — the player stops reading the Temperature as a clock and starts reading it as a consequence with a deadline they can move.

**Risk:** told plainly, "turn 19 is your last chance" may make turns 1 to 18 feel unpressured, the opposite of the intent. Pair it with #4, whose Tipping Points make the Last Turn get closer faster than the turn counter advances — watching that number come toward you is the whole effect.

---

## 10. Restoration Projects, and the Prospectors' mirror

**Medium**

Sharpen both signature rules from one-turn gestures into positions on the board. **Restoration** (Custodians, §14.2) stays as it is, but Restoration bought in **the same Nation State three turns running** becomes a permanent **Restoration Project** there: +1.0 ppm to the Natural Sink for the rest of the game, occupying one build slot, destroyed if the state changes hands or loses the slot to the sea. Its mirror, **Cheap Industry** (Prospectors), gains a second clause: a **Strip Permit** on a controlled state, free, one order, which doubles every Facility's output there for three turns and afterwards adds +0.2 to that state's Baseline Emissions and +3 Unrest, permanently. One Strip Permit per state, ever.

**Why it makes the game better:** both signature rules become things you build a map position around and can lose, and both create the same decision from opposite ends — is this state worth committing to for three turns?

**Risk:** the Restoration Project makes the Custodians' Stabilization run considerably easier at the same time as #1 does; these two want to be balanced together, not separately. Strip Permit also needs Unrest (#3) to exist or its downside is nearly free.

---

## 11. Coastal and inland build slots

**Medium**

Build slots in a Nation State stop being interchangeable. A state's slots are split by its Coastal Exposure: **Coastal Exposure many of them are coastal**, the rest inland, shown as two rows on the state card. Coastal slots are the ones Sea Level takes (§11.4), oldest Facility first; inland slots are never taken. Every slot added by raising Industry Level is inland. Building on a coastal slot is otherwise identical, so a state with free coastal slots and no inland ones offers a choice: build now on ground the sea is coming for, or raise Industry Level first and build safe.

**Why it makes the game better:** it makes Sea Level a thing you plan around rather than a tax you pay, and it gives the Prospectors a legitimately good aggressive play — build coastal, extract fast, and be off Earth before the water arrives.

**Risk:** it adds a second dimension to every Earth build decision, and the playtest note already flags "the moment you were not sure what to do next" as a worry; the state card must show it as two labelled rows or it will be invisible. It also mildly conflicts with the current "Build slots = Size + Industry Level" formula, which becomes a split rather than a total.

---

## 12. Antarctica melts under its own Colonies

**Small**

Antarctica's three Colony Slots (0.04 §5) answer to the Temperature like everything else on Earth. At each Sea Level threshold (+1.8, +2.3, +2.8 °C), **one Antarctic slot is lost**, in order: the Ross Ice Shelf, then the Antarctic Peninsula, then Lake Vostok. A Colony standing in a lost slot loses its Habitats; its Colonists are moved to the controller's most populous controlled Nation State, or lost if it holds none. The Climate Panel and the Antarctica surface view mark which slot goes next and at what Temperature.

**Why it makes the game better:** it closes a feedback loop the game already half-built — the cheapest colonies in the game sit on the one thing warming destroys, so the Prospector who burns Earth burns their own nearest landing ground first.

**Risk:** the 0.04 finding is that the AI founds no Antarctic Colony at all, so this may punish only the human player until the AI's slot-choice rule (§16.4) is revisited. It is also strictly a negative on a feature just added, and if Antarctica is meant as a friendly on-ramp for a new player, this takes that away.

---

## What the agent would build first

**#1, #4 and #9 together** are one small coherent change: the Custodians get a verb, the Temperature gets a shape, and the panel explains both. They do not touch the Influence system, the combat system, or the build queue, and they aim squarely at the standing 0.02 finding that no Faction ever meets its Victory Condition. **#2 and #5** are the two that most change what Earth-side play *means*, and both want the pacing sweep re-run in the same version. **#3 (Unrest)** is the prerequisite for #10's Strip Permit and #8's refugees having any weight, so if you want either of those, it goes first.
