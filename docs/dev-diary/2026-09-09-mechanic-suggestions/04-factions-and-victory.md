# Suggestions: Factions, asymmetry, victory and the endgame

Brainstormed by an Opus agent on 2026-09-09 after reading the spec, the glossary and the playtest note. Proposals only; nothing here is decided. Names in particular are placeholders.

---

# 1. The four remaining Factions

**The Arkwrights** — output ×1.0, Emissions ×1.0, Research ×1.0, Influence ×0.8, strength ×0.9, plus Habitat capacity ×1.5 and transit Fuel ×0.75; signature rule **Steerage** (a lift from a Launch Site carries twice the Colonists and costs its Nation State twice the population; Colony Ships cost 20 Materials, not 30); Victory Condition **Diaspora** — 30 Colonists in Habitats off Earth spread across at least three Bodies; in space because leaving *is* the win, and they are the one Faction that can meet its condition in the same End phase as the Collapse Line.

**The Syndics** — output ×1.1, Emissions ×1.0, Research ×0.9, Influence ×1.1, strength ×0.8, and a new Ducat row at ×1.5; signature rule **Freight Lanes** (each Trade Post pays +2 Ducats for every *other* Body where they hold a Colony or Space Station, so a thin wide network compounds); Victory Condition **the Long Ledger** — 1200 Ducats earned cumulatively, counted at production like an Extraction Total and never spent down, plus a Trade Post standing on four different Bodies; in space because the network is the score, which makes Phobos and Deimos worth a slot to them and nobody else.

**The Savants** — output ×0.8, Emissions ×0.9, Research ×1.6, Influence ×1.0, strength ×0.9; signature rule **Provisional Findings** (they already have half the effect of the Tech under research — a ×1.5 reads ×1.25 for them, a ×0.5 reads ×0.75 — so they live one Tech ahead of the board); Victory Condition **the Whole Tree** — every Tech on the Tech Tree complete, plus an Observatory (their exclusive Module, entry 8) on three Bodies; in space because the far Bodies are where the Observatories go, and because their win needs everyone's Research, so rivals are complicit in it.

**The Marshals** — output ×1.0, Emissions ×1.1, Research ×0.8, Influence ×0.9, strength ×1.25; signature rule **Martial Law** (Occupation Pacifies in two turns instead of three, and each turn of Occupation gains Standing equal to half the threshold rather than a third; Armies and Carriers cost half); Victory Condition **Dominion** — control five of the eight Nation States *and* hold Orbital Control at three of the five Bodies in the same End phase; in space because Orbital Control needs warships at three Bodies, which needs Shipyards, which needs Colonies or Space Stations out there.

Each takes an axis the existing two leave idle — Colonists, Ducats, Research and force — so the multiplier table gains only one new row (Ducats) and every signature rule reuses an existing subsystem rather than adding one. The four Victory Conditions vary the off-world half deliberately (Colonists on three Bodies, Trade Posts on four, Observatories on three, Orbital Control at three) while keeping the pillar that nobody wins on Earth alone. Two of them are cumulative like the Extraction Total, one is a state-of-the-board check like Stabilization, and Dominion is the only one that must be true simultaneously, which makes it feel different to play: you assemble a position and then have to hold it for one End phase.

**Why it is better:** it gives six genuinely different first ten turns instead of six flavours of build-and-expand, and it puts a Faction on the board whose win the others have to actively want to stop rather than merely out-race.
**Size:** Large (four Faction cards, one new multiplier row, four signature rules, four new Victory checks and four AI weight tables).
**Risk:** the Marshals' ×1.25 strength is the one number that can dominate, since combat is currently symmetric and the AI attacks at 0.6 odds — consider ×1.15 with the half-cost Armies doing the real work. The Savants' Provisional Findings needs every Tech effect expressed as a multiplier that can be halved toward 1, which a few (Hardened Hulls' flat +2, the card immunities) are not; rule those out explicitly.

# 2. The Blame Ledger

Track each Faction's cumulative Emissions from the sources it controls — the Climate phase already computes exactly this, per source, with the Faction multiplier applied. A Faction's **blame share** is its cumulative Emissions over the total, and its Influence thresholds on every **neutral** Nation State are multiplied by 1 + (share − even split), capped at ×1.5: at a 70% share in a two-Faction game a threshold of 50 becomes 60. Green Consensus, Clean Power and Clean Manufacturing lower blame as they lower Emissions, and the Custodians' ×0.75 already earns them a discount without a special case. The natural Prospector answer is to buy Influence with Ducats, to take places by Occupation instead, and to pay for Clean Power earlier than they otherwise would.
**Why it is better:** it makes "the world is watching" a rule rather than flavour, and it gives the dirty Factions a real strategic cost on Earth that they can plan around, instead of the current one-way street where the Custodian AI simply out-Allotments everyone.
**Size:** Small (one running total, one multiplier in the threshold function, one line on the state card).
**Risk:** it stacks on top of a Prospector seat that version 0.04 already reports as losing Earth; applying it only to neutral states and capping it at ×1.5 is deliberate, and it should be swept before the challenge margin is touched again.

# 3. Collapse with survivors

Keep the Collapse Line at +3.0 °C and keep the End-phase order (Victory Conditions checked first, then Collapse), but change what Collapse produces. Define a **Self-Sufficient Colony**: a Colony off Earth — not Antarctica, not a station over Earth — with a Habitat, a Generator, a Mine and a Refinery all online and at least six Colonists. When the Collapse Line is crossed, the game ends; every Faction without such a Colony loses outright, and among those that hold one the Faction with the most off-world Colonists takes a **Survival**, shown on the game-over screen as a lesser result beneath a true Victory. If nobody qualifies, nobody wins, exactly as today.
**Why it is better:** it turns the shared loss from a flat "everyone wasted their evening" into a last-minute race that the losing Faction can still play for, and it makes the four Modules of a real colony worth building for a reason other than yield.
**Size:** Medium (one predicate, one new game-over branch, one line on the Victory panel).
**Risk:** it slightly rewards letting Earth burn, which is thematically right for the Arkwrights and wrong for the Custodians — whose condition already requires Earth habitable, so they cannot take a Survival; state that explicitly rather than leaving it to be inferred.

# 4. The Climate Accord

In Orders, a Faction may propose an **Accord** naming a per-turn contribution (10 Energy or 20 Ducats, the Restoration rate) and a term of three turns; every other Faction answers accept or refuse before End Turn, and the AI answers from its weights. In each bound Climate phase every signer's contribution enlarges the Natural Sink at Restoration's rate — the contributions pool, so three signers at 10 Energy give +9.0 ppm — and every signer gains 5 Standing on every neutral Nation State. A signer that cannot or will not pay in a bound turn is a **defector**: it loses 10 Standing everywhere and its blame share (entry 2) counts double that turn. The Custodians' Restoration stacks with an Accord rather than replacing it, so they are the natural convener and the natural victim of a defection.
**Why it is better:** it is the one mechanic in this list that is about the actual subject of the game — collective action against a shared atmosphere — and in a six-Faction game it is where alliances form and break without a diplomacy screen.
**Size:** Medium (one proposal object, one accept/refuse popup, an AI acceptance rule, a defection check in the Climate phase).
**Risk:** with only two Factions it collapses into "the Prospectors always refuse"; it needs at least three seats to be interesting, so build it for the six-Faction game and not for 0.05.

# 5. Six different openings

Retire the identical starting position of section 14.3 and give each Faction a start package that makes turn 1 feel like a different game: the Custodians the ISS plus a Research Lab already standing in their state, the Prospectors Tiangong plus a Factory, the Arkwrights Mir plus a Colony Ship already in orbit but only 40 Materials, the Syndics Skylab plus a Bank and 60 Ducats, the Savants a station with the Shipyard already built but 60 Materials, the Marshals a second Army and a Carrier in orbit. Every package must leave the signature rule usable on turn 1 — Restoration needs 10 Energy against a start of 20, Cheap Industry needs 15 Materials — so the player's first End Turn already expresses the Faction. Total value is held roughly even in Materials-equivalent and stated in `factions.toml` so you can tune it by hand.
**Why it is better:** it answers "what does this Faction feel like on the board in the first three turns" with something the player can see and press, rather than a multiplier they will only notice on turn eight.
**Size:** Small (a start-package table plus one line per Faction on the New Game card).
**Risk:** the Arkwrights' free Colony Ship and the Savants' free Shipyard are worth 30 and 35 Materials respectively and skip a build turn each, which is more valuable than the raw number — expect a sweep.

# 6. A floor under the last-turn tiebreak

Today, if nobody meets a Victory Condition by turn 24, the higher percentage simply wins — and all three amendment documents report that in AI-versus-AI games nobody ever meets a condition, so every game ends this way. Add a bar (`victory.toml`, `timeout_bar = 0.6`): at turn 24 the higher percentage wins **only if it is at or above 60%** of its own condition; otherwise nobody wins and the game-over screen says the world ran out of time with nobody in reach. Pair it with the Victory panel showing the rival's standing as a band — Behind, On pace, In reach — until turn 20 and an exact percentage after, so the last four turns are the ones where you can see precisely how close the race is.
**Why it is better:** it converts a known balance failure from a silent default win into a loud, legible outcome, and it makes turns 20 to 24 the tensest of the game rather than a formality.
**Size:** Small (one comparison, one game-over string, one panel mode).
**Risk:** if your bars stay where they are, most games will end in "nobody wins", which is honest but bleak — the bar is a dial, and 0.5 may be the right first setting.

# 7. Hidden Mandates

Give each Faction three variants of its Victory Condition in `victory.toml` — for the Prospectors, say, 500 Extraction Total, or 350 with a Mine on four Bodies, or 250 plus 200 Materials-and-Fuel sold through the trading window. The player chooses among their own three on the New Game screen; every rival is dealt one at random and the Victory panel shows only its branch ("the Prospectors pursue an extraction mandate") and its overall percentage, naming the exact Mandate once the rival passes 50%. What the rival builds is the tell: four Mines on four Bodies reads differently from one enormous Mars operation.
**Why it is better:** it makes reading the board an actual skill and gives the same two Factions three openings each, which is most of the replayability of a 24-turn game.
**Size:** Medium (a Mandate table, a per-Faction selection, a redaction rule on the Victory panel, AI pace tables per Mandate).
**Risk:** in single-player, hiding too much makes the Victory panel useless — always show the rival's percentage even while its Mandate is hidden, or the player has nothing to steer by.

# 8. One exclusive building each

Give each Faction one building nobody else can place: the Custodians a **Sink Array** (Facility, no output, enlarges the Natural Sink a little every turn it is online — Restoration without the Energy bill), the Prospectors a **Strip Mine** (Module, double Mine yield, and the only Module that emits off Earth), the Arkwrights an **Arcology** (Module, holds 12 Colonists for 60 Materials), the Syndics an **Exchange** (Facility, lowers their trading-window buy prices by 1 each), the Savants an **Observatory** (Module, the only source of Research off Earth) and the Marshals a **Garrison** (Module, a Colony Army that may attack, which no Colony Army currently may). Each slots into the existing Facility or Module table with the same columns and the same build rules.
**Why it is better:** identity you can point at on the board beats identity in a multiplier table, and the Observatory in particular gives the Savants a reason to colonize that no other Faction has.
**Size:** Medium (six rows, six effects, six AI weights; the Strip Mine and the Observatory each break a standing rule and need care).
**Risk:** the Strip Mine contradicts "off Earth a Module still emits nothing" and the Garrison contradicts "a Colony's Army never attacks" — both are deliberate exceptions, but they are exceptions a player has to be told about.

# 9. The closing window

Change one line in the sea-level rule: when a Nation State loses build slots to a threshold, a **Launch Site is destroyed first**, before the highest-upkeep ordering takes over. Since a Launch Site is what lifts Colonists and Armies into orbit at all since version 0.04, every coastal state that drowns closes a door out of the world, and the Climate Panel's projection line should name the turn each state's Launch Site is expected to go. The result is that warming does not merely tax you, it takes away the escape route, and the decision "leave now or build one more Factory" gets a real deadline that the player set themselves.
**Why it is better:** it turns the Climate Model from a penalty meter into the thing that closes the window on every Faction's off-world half, which is the tension the whole game is named for.
**Size:** Small (one ordering change, one projection line).
**Risk:** it is harsh on a player who started in North America or Asia and never diversified; check that at least one low-exposure state is always reachable, and note that it hurts the Arkwrights most, which is either perfect or unfair depending on your taste.

# 10. Pacts and the Breach

For the three-plus-Faction game: in Orders a Faction may offer a **Non-aggression Pact** to one other for 20 Ducats, binding three turns if accepted. While it stands, neither may set Attack or Intercept against the other, nor spend Influence on a place the other controls. Attacking anyway is legal and is a **Breach**: every Standing the breaker holds drops 10 at once, and no Faction will accept a pact from it for five turns. The AI accepts when its threat score against the offerer is low and it is not within 20% of its own bar, and it will breach only against a Faction within one turn of winning — which makes the pact system self-policing around the leader.
**Why it is better:** it gives a single-player game a real negotiation surface — who you buy peace from, and when it is worth the reputational cost to break it — without a diplomacy screen or a trade UI.
**Size:** Large (offer and answer flow, a bound-order restriction, the Breach penalty, AI acceptance and breach logic).
**Risk:** binding Influence as well as attacks is strong and may freeze the board; consider making the Influence half optional in the offer.

# 11. The Second Chair

The Research Lead — the Faction that contributed most to the Tech just completed — currently picks the next Tech, every time, which compounds for whoever has the Research multiplier. Change it so that **every second completed Tech is picked by the Faction that contributed least**. The Savants' Whole Tree condition survives this untouched (they want everything researched, whoever picks), while the Custodians and the Prospectors stop being able to lock the tree away from each other for a whole game.
**Why it is better:** it removes the quietest snowball in the game and creates a genuine bluff — sometimes you *want* to be the low contributor on the Tech that is about to finish.
**Size:** Small (one alternating flag in the Research Lead check, one line on the tech panel).
**Risk:** it can hand the pick to a Faction that has contributed almost nothing all game, which may read as arbitrary; showing "next pick: the Prospectors (second chair)" on the tech panel from the start of each Tech fixes the surprise.

# 12. The Emergency Session

Once per game, on any turn between 8 and 20, the Faction with the lowest Victory percentage at the start of a turn may call an **Emergency Session** and take one of three boons: complete the Tech under research at once, take +30 Influence this turn, or receive a free Colony Ship at a Space Station it holds. It is announced in the Report to every Faction, so the leader can see it coming and spend a turn preparing.
**Why it is better:** it gives a player who lost a state on turn 9 a reason to keep playing to turn 24 without touching the underlying economy, and the announcement makes it a dramatic beat rather than an invisible handicap.
**Size:** Small (an eligibility check, a three-button popup, an AI choice rule).
**Risk:** this is rubber-banding, and rubber-banding in single-player reads as the computer cheating when the AI uses it — if that bothers you in play, restrict it to the player and call it a difficulty setting rather than a rule.
