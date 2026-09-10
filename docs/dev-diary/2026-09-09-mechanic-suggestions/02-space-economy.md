# Suggestions: the space economy, colonization and logistics

Brainstormed by an Opus agent on 2026-09-09 after reading the spec, the glossary, `bodies.toml` and the playtest note. Proposals only; nothing here is decided. Numbers given are starting values, not proposals to freeze.

The three problems the agent kept hitting in the spec: **a Colony is a yield multiplier, not a place** (four numbers on `bodies.toml` are the entire difference between Tycho and Olympus Mons); **distance costs nothing after arrival** (a Mine on Phobos feeds an Earth build the same turn it produces, and Energy generated on Deimos runs a Factory in Asia); and **the space half of the game has no clock of its own** — the Climate Model paces Earth, but Mars is always four turns and 20 Fuel away, every turn of the game. Ideas 1 to 5 attack those three directly. The 0.04 open items say the AI colonizes nothing but Mars, builds no station beyond its start one and never touches Phobos, Deimos or Antarctica; that is the same problem seen from the AI's side, and several of these fix it by giving those Bodies something Mars does not have.

---

## 1. Every Colony Slot a named site

**Give each Colony Slot its own multipliers on top of the Body's.** Version 0.04 made every slot a real geological place with a name and a position; let the place do something. Each row in `bodies.toml`'s `slots` gains an optional multiplier per Module class and one line of flavour text on the Colony Slot panel: Tycho is young ejecta (Mine ×1.25, Habitat ×0.9); the South Pole-Aitken Basin has shadowed ice (Refinery ×1.75, Generator ×0.6 — no sun at the pole); Valles Marineris is sheltered and deep (Habitat ×1.25, Mine ×0.9); Olympus Mons is high and thin-aired (Generator ×1.3, Refinery ×0.75); Lake Vostok sits on liquid water (Refinery ×2.0, Habitat ×1.25); Stickney is the crater that nearly broke Phobos (Mine ×1.5 on an already-1.75 Body). These multiply into the existing chain exactly where the Body yield already does (spec 7.1: Body yield, Resource Lean, Faction multiplier, Tech multipliers), so no new arithmetic exists — only a new table column.

**Why:** founding becomes a real choice with a name attached instead of picking whichever free slot is on the best Body, and two Colonies on the same Body stop being copies of each other.

**Size:** Small. Data table plus one multiplier lookup; the AI's Colony Slot rule (16.4) needs to read the slot multiplier instead of only the Body's.

**Risk:** multipliers compound with Body yield, so Lake Vostok's Refinery ×2.0 on Earth's ×1.5 is ×3 before Automated Refining — keep the spread inside roughly 0.6 to 1.5 or the best slot on a Body becomes the only slot worth taking, which is the failure this is meant to cure. Also: Antarctica's Modules emit (0.04 §5), so a strong Vostok Refinery is a climate decision, which the agent thinks is a feature.

---

## 2. Energy stays where it stands

**Take Energy out of the shared Stockpile and make it a balance per Body.** The glossary already defines Energy as "what runs a mine, habitat or industry **where it stands**" — the shared Stockpile contradicts its own definition. Energy would be produced, held and spent per Body: Earth's Power Plants and Antarctica's Generators feed one Earth balance, the Moon's Generators feed the Moon's, and a Space Station's Generators feed its Body's balance. The Income shortfall rule (7.2) is unchanged in every particular; it simply runs once per Body instead of once globally, so a Colony that outgrew its Generators shuts its own Mine down and Earth never notices. The Trading window sells Energy into Earth's balance only, Restoration spends Earth's Energy, and Ships and Armies draw their upkeep from the Body they are at. Materials, Fuel and Ducats stay shared, read as freight and finance that the game abstracts.

**Why:** it is the single change that makes a Colony a place that has to be built out rather than a socket that draws from home, and it turns Generator yield — currently the least interesting number in `bodies.toml` — into the first thing you look at before founding.

**Size:** Medium. The shortfall rule and the top bar both need a Body dimension; the AI's "+2 to an Energy producer while the balance is within one turn's upkeep of zero" rule becomes per-Body, which is a small change that improves its play a lot.

**Risk:** it makes early Colonies markedly harder (a Colony's free founding Habitat costs 2 Energy with nothing producing), so the first build at a new Colony is always a Generator and founding wants a Colony Ship's worth of Materials behind it — that is either the good tension or a tax, and it is your call which. It also promotes Closed-Loop Colonies to a near-mandatory Tech, and Reactor Leak (0.02) and Grid Failure become far more dangerous cards. Phobos and Deimos at Generator 0.75 and 1.0 get harsher still, so pair this with idea 12 or they stay uncolonized.

---

## 3. Launch windows on a turning map

**Give Mars an orbital position that moves, and price the transit off it.** The Solar System Map already draws the Bodies on a plane; let Mars advance around it a fixed number of degrees each turn on a cycle of, say, eight turns, so a 24-turn game has three windows. At the window, a transit is the card's 4 turns and 20 Fuel; at the far side of the cycle it is 6 turns and 32 Fuel; between, it interpolates and the panel states the exact figure before you commit. Phobos and Deimos ride Mars's number, the Moon and Antarctica are always open, and the top bar or the Solar System Map carries a small dial reading "Mars window in 3 turns — 20 Fuel; today 29". Efficient Transit's ×0.6 applies after, so the Tech is worth most when you are forced to fly off-window.

**Why:** it gives the space half of the game a rhythm of its own to plan against, and creates the best decision in the whole economy — go now at double Fuel, or spend three turns building and go cheap while your rival is already there.

**Size:** Medium. One position value per Body advanced in the End phase, a cost function, a dial, and a real change to the AI: the greedy scorer of 16.1 will always launch at the worst moment unless it gets a rule to bank Fuel when a window is within two turns.

**Risk:** dead turns. If a player misses a window they may have three or four turns with nothing to do in space, which is exactly the boredom the playtest note is fishing for. Mitigations worth considering: never let the off-window cost exceed 1.6× (so waiting is a preference, not a wall), and keep the Moon always open so there is always something to fly. It also interacts with Solar Storm (no transit advances) — a card drawn on the window turn is brutal, which may be a memorable moment or an unfair one.

---

## 4. Colonists born off Earth

**Let a Colony grow its own Colonists instead of importing every one.** In the Income phase, a Colony with at least two Colonists, free room in its Habitats, and no Energy shortfall in either of the last two turns gains one Colonist every N turns, with N scaled by the Body's Habitat yield (Mars at 1.5 grows fastest, Phobos and Deimos at 0.5 barely at all). Growth stops the moment Habitat room runs out, so the loop is Habitat, fill, Habitat again, and the arc of a Colony becomes founded → dependent → self-sustaining. Because Off-world Presence is 12 Colonists and today every one of them must be lifted from a Nation State and shipped, this changes the shape of the Victory Condition from a shipping schedule into an investment: seed a Colony early and it pays you Colonists for the rest of the game.

**Why:** it is what makes a Colony feel alive rather than warehoused, and it gives Off-world Presence a second route so the game is not decided by how many Colony Ships you built in the first eight turns.

**Size:** Medium. One growth rule in Income, a "growing / full / stalled" line on the Colony card, and the AI needs to value a Habitat at a growing Colony above one at a dying Body.

**Risk:** it directly moves the pacing of both Victory Conditions, and 12 Colonists may become too easy — expect to raise the bar or slow N after measuring. It also interacts with the Antarctica rule (0.04 §5): Antarctic Colonies would grow too, at Habitat 0.75, and their Colonists still count for nothing off-world, which is probably right but should be said out loud on the card so nobody feels cheated.

---

## 5. A warming Earth fills the ships

**Make emigration easier the worse Earth gets.** For every full 0.2 °C the Temperature stands above +1.8, a Colony Ship loading Colonists at a Launch Site takes one extra Colonist beyond its capacity (cap the bonus at +4), and the population cost to the state stays 0.1 per Colonist as now. Nothing else changes: the lift is still a launch for Emissions (0.04 §7), still needs a working Launch Site in a controlled state, still needs Habitat room at the far end. The Report and the Launch Site panel say plainly why: "+2.4 °C — 6 Colonists ready to leave Asia this turn."

**Why:** it closes the loop the game is named for — the dying Earth is what makes the colonies possible — and it puts a genuinely uncomfortable decision in front of a Prospector who is behind on Off-world Presence and holds the throttle on the Emissions that would help.

**Size:** Small. One term in the load rule (7.4), one line on two panels.

**Risk:** it rewards warming, and a Prospector AI running the denial multiplier (16.3) will find it fast; the counterweight is that the Collapse Line ends the game with nobody winning, so the strategy has to stop somewhere. It also quietly helps the Custodians' Stabilization run, since population emits 0.1 per hundred million and emigration reduces population — worth checking in a sweep before adopting, because it may be a bigger effect than it looks.

---

## 6. Propellant depots and the tanker run

**Hold Fuel per Body, the way Energy should be held per Body.** A transit spends the Fuel of the Body it departs from. Refineries fill their own Body's depot; the Trading window sells Fuel into Earth's depot only; Fuel moves between Bodies only aboard a Ship, with a Carrier able to carry 10 Fuel in place of its Army (or a fifth Ship type, a Tanker, if you would rather keep the Carrier clean). Every Body's depot shows on the Solar System Map beside its stack markers. This is what finally makes Mars's Refinery yield of 1.5 strategic rather than decorative: you refine at Mars and the 2-Fuel hops to Phobos and Deimos become nearly free, while a rival flying everything from Earth pays 24 Fuel a leg.

**Why:** it is the supply line — the moment Fuel has a location, the map acquires a forward base, a long leg and a place worth blockading.

**Size:** Large, and the largest change on this list. Every transit order, the Trading window, the AI's transit scoring and the Report all gain a location.

**Risk:** stranding. A Ship at a Body with an empty depot cannot move at all, and a player who empties a depot by accident has lost a Ship for good with no message telling them why — this needs a confirm-and-warn on any order that would leave a Body below one transit's worth. The agent would not adopt this and idea 2 in the same version; pick whichever localization you want the game to be about, and note that this one bites hardest exactly where idea 3's launch windows also bite, so the two together may be too much friction at once.

---

## 7. Build it where you dig it

**A Module or Ship built at a Body where the building Faction has an online Mine costs Materials ×0.75, and ×0.6 with two or more online Mines there** (a Body-level in-situ discount, capped, and stacking multiplicatively with nothing else). The Materials still come from the shared Stockpile — nothing about the Stockpile changes — but the fiction is that regolith you are already digging does not need lifting. The effect is that the first Colony on a Body is dear and every one after it is cheaper, so a Body you have invested in starts to pay for its own expansion, and a Shipyard on a station over a Body with your Mines becomes the cheap place to build a fleet.

**Why:** it gives you a reason to deepen a Body rather than always spreading to the next one, and it is the cheapest possible way to make distance matter without unsharing the Stockpile.

**Size:** Small. One multiplier at the build order, one line on the build button's hover (0.02 §6 already shows three figures there).

**Risk:** it compounds with the Prospectors' ×1.25 output and Cheap Industry, so measure whether the Prospector's second Mars Colony arrives absurdly early. It also weakens Antarctica, whose Mine yield is 1.0 and which is already the least attractive place to colonize — you may want the discount to exclude Earth, or to apply on Earth as a small bonus to Facility builds in a state with a Materials Lean.

---

## 8. Two Modules only a station can hold

**Give Space Stations something to be besides a Shipyard.** A station today holds a Shipyard and Habitats and nothing else, which makes 40 Materials for a bare orbital slot a hard sell — the 0.04 open items note the AI never builds a second one. Add two Modules buildable only on a station: a **Solar Array** (25 Materials, 2 turns, no upkeep, produces 6 Energy and ignores the Body's `generator_yield` entirely, because orbit has unshadowed sun) and a **Fuel Depot** (30 Materials, 1 turn, 2 Energy, which reduces the Fuel of every transit departing that Body by a quarter). Both fit the existing rule that a station has no Body yield on its Habitats, and both make Orbital Slots at the poor Bodies — Phobos, Deimos, a shadowed Moon — worth taking.

**Why:** it turns an Orbital Slot from a shipyard socket into a piece of infrastructure worth fighting for, which is what makes Orbital Control and Intercept interesting.

**Size:** Small. Two rows in `modules.toml` plus a station-only flag.

**Risk:** a Solar Array that ignores Body yield is strictly better than a Generator at any Body with a yield under 1.0, which is most of them — that is intended, but it means idea 2's local Energy would route through stations and make orbital combat decisive in a way you may not want. The Fuel Depot overlaps with Efficient Transit; keep them multiplicative and modest, or one makes the other pointless.

---

## 9. Prices that move when you buy

**Make the Trading window a market rather than a table.** Every 10 Materials bought in a turn raises the Materials price by 1 Ducat, for that turn and the next; every 10 sold lowers it by 1; each price drifts one step back toward its table price each turn, and is floored at half and capped at three times the table figure. Fuel and Energy move the same way on their own tracks; Influence at 2 Ducats stays fixed, since it is a Faction's own Allotment and not a commodity. The window shows the current price, the next price and the drift, so a lot of forty Materials is visibly the last ten costing double.

**Why:** it fixes the uncapped-window problem the 0.04 document flags without a hard cap, and it makes hoarding Ducats for one big turn a real play with a real cost.

**Size:** Small to Medium. Four price tracks in state, a panel change, and the AI's "buys Materials in lots of ten" rule must re-price between lots inside its greedy spend.

**Risk:** it is a nerf to the seat that holds North America and Asia, which may be the seat that needed the help; and price state has to survive being shown correctly mid-Orders when orders can still be cancelled and fully refunded — a refunded purchase must move the price back, or cancelling becomes an exploit.

---

## 10. Trade goes between places, not inside one

**Change what a Trade Post pays.** Today it pays 3 × the Body's Habitat yield, so it is a slightly worse Bank that happens to be off Earth, and the AI never builds one. Instead pay a Trade Post **2 Ducats per Colonist at its own Body plus 3 for every other Body where its Faction holds a Colony or a Space Station** — trade is a network, and one node earns nothing. A Faction spread over Earth, the Moon, Mars and Phobos would find a single Trade Post on Mars paying handsomely, while four Trade Posts stacked at one place pay almost nothing.

**Why:** it pays you for the shape of your empire rather than its size, and it is a direct answer to the AI colonizing nothing but Mars, because the second Body suddenly has a number attached.

**Size:** Small. One formula, one AI weight to retune.

**Risk:** Ducats are already the flexible resource (they buy Influence, Restoration, repairs and now whole buildings at twice cost), so a strong Trade Post is a strong everything; measure the Ducat income curve before and after. Note also that a Space Station counts as presence under this rule, which makes a 40-Materials bare station a cheap way to farm the network bonus — you may want to require a Colony, or require Colonists aboard.

---

## 11. Venus, a Body of orbits only

**Add a Body with no Colony Slots at all.** Venus: 3 turns and 16 Fuel from Earth (nearer than Mars, and on a different phase of idea 3's window cycle so it is available when Mars is not), zero Colony Slots, three Orbital Slots, and Colonists who live only on Space Stations and count fully toward Off-world Presence. No Mine, no Refinery, no ground of any kind — everything at Venus is built and held in orbit, and Orbital Control there is not a prelude to a landing but the whole contest. It costs almost no new rules: a Body with zero Colony Slots and a Space Station that holds Habitats are both already in the spec. Beyond it, the ladder the agent suggests for later versions is **Ceres** (far, Refinery-dominant, the water depot for anything outward), **a single named asteroid** (one slot, Mine 2.0, nothing else worth having), and the **Jovian moons** as a late frontier whose Colonists are worth double toward Off-world Presence.

**Why:** it gives the map a Body with a genuinely different shape rather than a fourth set of yield numbers, and it makes Space Stations — currently the thinnest piece in the game — the point of a whole destination.

**Size:** Small to Medium. Mostly data, plus handling a Body with no Colony Slots in the Surface Map and in the AI's slot-picking rule, which currently assumes there is a slot to pick.

**Risk:** a Body reachable only by station is a Body with no Barracks and no Colony Army, so it can only be defended in orbit — check that this does not make Venus a free gift to whoever brings the first Frigate. And a Surface Map for a Body nobody can land on needs a reason to exist, or Venus should open straight to its orbital band.

---

## 12. A mass driver on a small world

**Give the low-gravity Bodies the one thing they are actually good for.** Add a `low_gravity` flag to `bodies.toml` (the Moon, Phobos, Deimos) and a **Mass Driver** Module buildable only there: 40 Materials, 2 turns, 4 Energy, and while it is online every transit departing that Body spends 4 less Fuel (minimum 1) and each Mine at that Colony produces +1 Materials. Phobos, at Mine 1.75 and one turn from Mars, becomes the ore rock and the cheap departure point for everything outward; Deimos becomes the quiet depot. Today those two Bodies have three Colony Slots between them and, per the 0.04 open items, the AI has never founded on either — because Mars beats them on every number that exists.

**Why:** it gives the small Bodies a role that Mars cannot take from them, which is what specialization between Bodies actually means.

**Size:** Small. One Module row, one Body flag, one term in the transit Fuel calculation.

**Risk:** it overlaps with Efficient Transit and with idea 8's Fuel Depot — three separate Fuel discounts multiplying is one too many, so pick two at most. And a flat −4 Fuel is worth far more on a 6-Fuel Moon hop than on a 24-Fuel outer leg, which is backwards from the intent; a percentage would be better behaved but reads less like a machine that throws cargo off a small world, and that flavour is half the point.

---

## If you only take three

Ideas **1**, **2** and **3**: named sites, local Energy, moving windows. Together they answer the three questions in the brief — a Colony becomes a place with character, a Body becomes something you have to build out rather than plug into, and distance acquires a clock. Idea 1 is nearly free and the agent would do it regardless. Ideas 6 and 3 both add friction to the same part of the game, so the agent would not ship them in the same version.
