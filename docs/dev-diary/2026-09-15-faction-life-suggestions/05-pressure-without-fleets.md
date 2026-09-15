# Suggestions: the ways Factions collide that are not armies

Brainstormed by an Opus agent on 2026-09-15 after reading the glossary, the 0.07.0 and 0.08.0 specs and the engine. Proposals only; nothing here is decided.

Today two Factions meet in three ways: they bid Influence against each other on the same Region or Colony, they fight Battles, and they race for the Research Lead. Everything below is pressure in between, and each entry names the existing system it hangs off, because this project prefers deepening an existing rule to bolting on a new one.

---

## 1. The Runner-up strikes one

**What it is.** When a Tech completes, `research.contributions` already names every seat's share and picks the Research Lead from it. Give the **second** contributor one strike: before the Lead picks, one Tech is struck from the Shortlist of three, leaving two. No order, no cost, no new phase — it falls out of the array the engine already holds. The Lead's own Victory gate is never strikable, so the rule of version 0.07.0 stands.

**Hangs off.** The Research Lead and the Shortlist (`engine/src/research.rs:140`, `draw_shortlist`).

**Why it brings life.** Every point of Research now buys something even when you cannot lead, and the Archivists' standing choice to Fund the Archive costs them the strike as well as the Lead — one decision, two prices.

**Size.** Small: the Shortlist is already drawn in one function, and the runner-up is a `max` over the same array the Lead comes from.

**Risks.** A computer seat will strike by whatever heuristic `ai_tech_pick_with_reason` is given; if that heuristic is naive the human is struck cleverly and strikes back stupidly.

**Open design questions.** Does the strike happen automatically (*the agent recommends* yes — an order would need a window between the draw and the pick, and the draw happens mid-Resolution) or as a choice a human runner-up makes in the Orders phase? Is the strike blind, or does the Lead see what was taken away? Should the runner-up be barred from striking two Techs running?

## 2. Agitate

**What it is.** Relief's exact mirror: an Orders-phase order, paid in Ducats, on a Region a **rival** controls, raising its Unrest by one. As many times a turn as you can pay for, cancellable like any order, resolved in `resolve_unrest` beside Relief. Past the second Unrest threshold that state's Facilities run at half; at the top it throws its controller off and goes neutral — so the money buys a rival's industry and, eventually, his Region.

**Hangs off.** Unrest and Relief (`Order::Relief`, `engine/src/resolution.rs:1692`), paid in Ducats.

**Why it brings life.** Unrest stops being weather and becomes a weapon, and Relief stops being a chore and becomes a defence against somebody.

**Size.** Small: one order variant, one `Pending` vector, one Report line, one AI appetite.

**Risks.** This is the sharpest "punishes the player more than the computer seats" case in the list — a human will agitate every turn and the AI will not unless `ai.rs` gets a matching valuation, and a cheap Agitate makes unseating a computer Faction a Ducat sum rather than a campaign.

**Open design questions.** Does Agitate count as an offending turn for Relations (*the agent recommends* yes — it is precisely what Relations should read, and `offended` is already a flag)? Does the victim's Report name who paid, or only that Unrest rose? Should a Constabulary blunt it the way it blunts the climate?

## 3. Table prices that move

**What it is.** `per_materials`, `per_fuel` and `per_energy` are fixed constants in `ducats.toml`. Make each move with what the four Factions bought and sold last turn: net buying raises a price a step for the next turn, net selling lowers it, drifting back toward the card figure when the table is quiet. Shown on the Trading window with last turn's move.

**Hangs off.** The Trading window (`Order::Buy` / `Order::Sell`, `engine/src/orders.rs:259`).

**Why it brings life.** Rivals meet in the market without any order naming a rival; buying out the Fuel before a Launch Window becomes a real play, and dumping Materials becomes a way to make a rival's build dearer.

**Size.** Small: one figure per resource recomputed at Income, saved, and drawn.

**Risks.** If the computer seats trade rarely, the only hand on the price is the player's and the rule taxes him alone — measure AI trade volume before committing to the step size.

**Open design questions.** How far may a price move in a turn, and is there a band? Does it drift back, and how fast? Is last turn's volume public, or only the price (*the agent recommends* publishing the price and hiding the volume — the price is the market speaking, the volume is espionage)?

## 4. Blame rots your claims, and Blame Credit pays

**What it is.** Blame has one mechanical job today: it raises a Faction's Threshold on every place it does not hold. Give it a second — **Standing decays faster** on places you do not hold as your Blame share rises above a fair quarter (2 a turn becomes 3). And give **Blame Credit** its first job at all: `blame_credit` is computed in `state.rs:2086` and drawn in the interface, but no rule reads it. A Faction in credit decays a point slower, or reads a lowered Threshold.

**Hangs off.** Blame, Standing decay (`engine/src/resolution.rs:768`) and Threshold.

**Why it brings life.** It answers the commons question: a Prospector free-riding on a Custodian's Scrubbers pays for it in claims that will not hold, and the Custodian's Scrubbers finally buy something outside their own Victory Condition.

**Size.** Small: one multiplier in the decay loop, one in the threshold.

**Risks.** It stacks with the threshold multiplier already in force and could make the Prospectors' Earth game unplayable — the two effects want measuring together over a sweep, not separately.

**Open design questions.** Should the two Blame effects share one budget so the total penalty is unchanged and only its shape moves? Does Blame Credit lower the Threshold, slow the decay, or both? Should a Faction in credit be visible to the others as such?

## 5. The Claim

**What it is.** Influence may be spent on a **free Colony Slot or Orbital Slot**, where it becomes Standing on the slot. The Faction holding the largest Standing at the slot's threshold has precedence: another Faction's Colony Ship may still found there, but it must beat the Claim as it would beat a controller, margin and all, and a beaten Claim's Standing is not wiped. Ordered from the Body Surface Map, where every slot and every ring is already drawn and clickable.

**Hangs off.** Influence, Standing and Threshold — but `Target` is `Place`, which is `State | Colony`, so it needs a third variant.

**Why it brings life.** The race to Mars becomes contested six turns before a hull arrives, and a rival's intent becomes readable on the board rather than a surprise at a Resolution.

**Size.** Medium: a new `Target` variant touches Influence orders, decay, the save file, the Roster, the AI and the Report.

**Risks.** A rich seat could claim every good slot on the board and make the landing game arithmetic rather than logistics.

**Open design questions.** Does a Claim bar a founding outright, or only make it dear (*the agent recommends* only dear — an outright bar takes the ship out of the game, and this project's rules have consistently preferred a price to a gate)? Does a Claim survive the founding, converting into Standing on the new Colony? Is it the obvious thing to trade away in diplomacy?

## 6. The Relay as an eye

**What it is.** A Relay raises Influence and Standing where it stands. Give it a second clause: while it stands and is online, its holder reads the **building-by-building breakdown** of any rival's income at that Body — the thing the Faction window explicitly withholds ("a rival's income is shown as totals only") — and, on Earth, an Embassy does the same for its Region.

**Hangs off.** The Relay, the Embassy and the Faction window (0.08.1 section 1).

**Why it brings life.** The Faction window becomes something earned rather than something read, and a Relay becomes worth building somewhere other than where your own people are.

**Size.** Small to medium: the figures all exist; this is a visibility gate and a tooltip, not new arithmetic.

**Risks.** The computer seats read the whole board regardless, so any fog rule taxes the player alone — which is why this must only *widen* what the player sees and never narrow today's default.

**Open design questions.** Should the Relay also reveal a rival's **committed orders** at that place one turn early, or only its income? Does the rival learn it is being read? Does this deserve its own Tech on the Society branch rather than riding free on a Module?

## 7. Censure

**What it is.** An Orders-phase order, once a turn, naming one rival: for the next turn that rival's Blame share is read as higher than it is, so its Threshold rises on every Region it does not hold. `blame_threshold_multiplier_on(seat, target)` already takes both arguments, so the hook is a single addend. Paid in Ducats, or in Influence from the Allotment.

**Hangs off.** Blame and Threshold (`engine/src/state.rs:2070`).

**Why it brings life.** Blame becomes something one Faction can point at rather than a private number, and the dirtiest seat is punished by a rival's choice rather than by arithmetic alone.

**Size.** Medium: small in the engine, but it needs a place in the interface to name a Faction, which nothing today does.

**Risks.** Three seats censuring the leader every turn is a pile-on that the computer players will not know how to join or resist.

**Open design questions.** Should Censure cost the accuser in Relations with the named Faction (*the agent recommends* yes, and calls it the clearest example of a pressure mechanic that diplomacy should later be able to trade away)? May a Faction with a worse Blame share censure at all? Is it public, or does the victim only see its Threshold move?

## 8. The Consignment

**What it is.** Sell Materials or Fuel to a **named rival** at a price you set, instead of to the table at half. The offer stands in the buyer's next Orders phase; taken, the resources move and the Ducats move back. It is the first place two Factions agree on anything, and it is the obvious seed for the diplomacy already on the roadmap.

**Hangs off.** The Trading window's `Sell`, and the Stockpile.

**Why it brings life.** Half is a terrible price; a rival who needs Fuel this turn is a better one, and a Faction with a Refinery surplus finally has somebody to be useful to.

**Size.** Large: it needs an offer that survives a turn boundary, a save field, an interface for making and answering one, and an AI that can value a resource it did not plan to buy.

**Risks.** A computer seat that values offers badly is either a bottomless purse or a wall, and either makes the whole mechanic dead weight.

**Open design questions.** Should this wait for diplomacy proper rather than arriving alone (*the agent recommends* yes — it is the cheapest thing to build *on top of* a diplomacy layer and the dearest to build beside one)? Does a Consignment repair Relations? May a Faction consign to a rival it is in a Battle with?

---

**The agent's own "if you only take one":** the Runner-up strikes one. It is a handful of lines in one function that already holds every number it needs, it costs no new order, no new interface and no new save field, and it turns the Research Lead from a prize one Faction wins into a bargain two Factions strike. Everything else here needs measuring against the computer seats first; this one needs measuring against nothing.
