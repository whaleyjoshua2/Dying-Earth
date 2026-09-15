# Suggestions: reading the rivals — what the player knows, when, and how they feel it

Brainstormed by an Opus agent on 2026-09-15 after reading the glossary, the 0.07.6 and 0.08.1 specs, `engine/src/report.rs`, `engine/src/victory.rs` and the interface. Proposals only; nothing here is decided.

**The agent's framing.** The board is already almost entirely open: the Report's rival paragraph names *every* order a rival committed (`assets/data/report.toml` `[rival]`, about 45 templates), the Solar System Map draws each transit with owner, destination and turns left (`src/ui.rs:2177-2185`), and the Victory window prints all four seats' progress to the unit. The only thing withheld is a rival's income breakdown. So the problem is **not** that the player cannot find out — it is that everything arrives flat, at the same volume, in a window they must go and open. Almost every suggestion below is about *weight and timing*, not new disclosure.

---

## 1. The rival's Moment

**What it is.** A ninth `MomentKind`: a rival crossing a step of its own Victory Condition — half, three quarters, and *one turn from meeting it*. One sentence and one number, in the rival's colour, before the Report, exactly as `ArchiveComplete` reads now. "The Archivists have uploaded 9 of 12 Colonists into the Archive."

**Why it brings life.** The game has eight Moments and not one of them is about losing. A Faction becomes a character the first time it interrupts you.

**Size.** Small — `MomentKind` (`engine/src/report.rs:157`) is a closed enum with a card in `report.toml` and a severity rank; `progress()` already gives the figure. One new saved field per seat (the highest step announced) so it fires once.

**Risks.** Fires four times a turn late on if the steps are too fine; keep it to two or three steps and let the existing two-a-turn cap and the Moments corner switch handle the rest.

**Open design questions.** Which steps? Should the *one turn from* warning exist at all, or is a silent loss the honest one? *The agent recommends* half, three quarters, and one-turn-from — the last being where the whole value is.

## 2. The rival paragraph's first sentence

**What it is.** The Report's rival section currently renders one run-on paragraph per seat with deeds in order of the order list, so "bought 3 Materials" sits beside "founded Ceres Station". Give each paragraph a **lead sentence**: the single most serious deed, by the same severity ranking the headline uses, then the rest as now.

**Why it brings life.** The rivals stop reading like a ledger and start reading like four people with a plan.

**Size.** Small — a rank on the deed keys and a sort in `Game::faction_paragraphs`; the templates need no change.

**Risks.** None to the game's depth; the risk is only that the ranking is wrong at first and a Mine leads over a Colony founded.

**Open design questions.** Does the lead deserve its own line in the Faction's colour above the paragraph, or just to be moved to the front? *The agent recommends* its own line — the paragraph is long and the eye needs a hook.

## 3. The standings strip

**What it is.** A four-seat row on the top bar beside the Research race bar: each Faction's symbol, a short bar of its Victory progress in its own colour, the leader's bar brighter. Clicking a symbol opens its page in the Faction window. Always visible, never opened.

**Why it brings life.** The race is currently something you have to *ask* about. This makes it ambient — you feel yourself slipping without pressing V.

**Size.** Small — `progress().score()` for four seats, the bar-drawing and `faction_link` helpers both exist; the Research race bar is the template.

**Risks.** Flattens a little: it reduces four different Victory Conditions to one comparable percentage, which is exactly the reduction the Victory window already makes. It buys tension at the price of nuance.

**Open design questions.** Percentage bars, or bare ordinal positions (1st–4th)? Should your own seat be marked? *The agent recommends* bars, with your own seat underlined rather than recoloured.

## 4. Works in hand

**What it is.** A block on the Faction window under Holdings: what this Faction has **under way** — builds begun and turns remaining, Ships in transit with their names and destinations, Emigrants waiting. This is *intent*, as distinct from position, and every figure already exists on the board.

**Why it brings life.** Holdings says what a rival has; this says what it means to do next. It is the difference between a rival and a scoreboard row.

**Size.** Small to medium — all state is in `Game`; the work is the block, and deciding what a rival may show.

**Risks.** Buries the player in numbers if it lists everything. Cap it at the largest few, or group by kind.

**Open design questions.** Does a rival show *all* its works, or only those the board could see (a Ship in transit is visible; a Facility begun inland arguably is not)? This is the one place the disclosure rule of ticket #203 genuinely bites again. *The agent recommends* showing it all, consistent with the Report already naming every build, and reserving the withholding argument for the income breakdown alone.

## 5. The Victory history

**What it is.** A sibling of the Emissions history, drawn on the Faction window under Victory progress: one Faction's progress turn by turn against the in-game date, the Archive's completion or Antarctica's opening ticked on the axis. A second line for Blame. Same house style as `emissions_history` / `population_history`.

**Why it brings life.** Shape, not position: a Faction that has been flat for ten turns and suddenly climbs is a story the current bar cannot tell.

**Size.** Medium — `EmissionsRecord` (`engine/src/state.rs:465`) records nothing per seat, so a new record and a `#[serde(default)]` on the save are needed before anything can be drawn. The drawing itself is a near-copy of an existing function.

**Risks.** Low; it is a record of what already happened. The cost is the new saved record, not the reading.

**Open design questions.** Four lines on one pair of axes, or one Faction at a time on its own page? Blame on the same axes or its own? *The agent recommends* all four on the Faction window's page for comparison — one seat at a time wastes the chart's only advantage. The axis carries the in-game date, per the standing rule.

## 6. The challenger line

**What it is.** On a Region card or Colony card you hold: one line naming the rival with the highest Standing against that place and how far it is from the price — "The Prospectors stand at 31; they take this at 54." The arithmetic (threshold, the challenge margin, Blame's multiplier, Resistance) is all in `Game` already.

**Why it brings life.** This is the single largest source of *arbitrary* defeat in the game today: a place changes hands and the first the player hears is a Moment after the fact.

**Size.** Medium — the figures exist but the per-seat threshold read is fiddly, and the line wants a hover explaining which of the four numbers moved.

**Risks.** The clearest flattening risk on this list: it turns Influence from a judgement call into an arithmetic one, and a player who reads it will always fund exactly enough and never a point more.

**Open design questions.** Does it show the exact figure, or only a band ("contested", "at risk")? Every place you hold, or only the most threatened? *The agent recommends* a band rather than a figure — it removes the surprise without removing the judgement, which is the whole balance this list is about.

## 7. The reckoning

**What it is.** The game-over modal (`src/ui.rs:6243`) lists four seats in seat order with no ranking. Sort it by the ranking the rules actually use — percentage of own Condition, then Colonists off Earth, then Colonies held — and show the chain, with the tiebreak that decided it named.

**Why it brings life.** A loss you can read the reasoning of is a loss you argue with. The rules already define this order in `victory.rs:130-142` and the screen ignores it.

**Size.** Small — a sort and a few lines of text against figures already computed.

**Risks.** None.

**Open design questions.** Show the two tiebreak figures for every seat, or only where they actually decided? *The agent recommends* every seat, with the one that decided marked.

## 8. Delayed sight

**What it is.** The design question rather than a feature: today a rival's Victory progress in the Faction window is *live to the instant*. The alternative is that it reads **as of the last Report** — a dated figure, one turn stale, stamped with its date. Nothing hidden, only aged.

**Why it brings life.** It makes the Report the thing that tells you where you stand, gives the endgame a genuine tremor of uncertainty, and costs the player nothing they could not work out.

**Size.** Medium — a per-seat figure snapshotted at the Report phase, saved, and read by the Faction window and Victory window.

**Risks.** Falls on the "too little" side: a player could lose to a turn they were never shown. Mitigated by the Moment in suggestion 1, which fires on the real figure.

**Open design questions.** This is squarely the designer's call, and the whole shape of the answer to "does this game want imperfect information at all". *The agent recommends* **not** doing it — this is a board game and the board is open; buy tension with 1 and 3 instead, which add weight without subtracting truth. But it holds that the question deserves a deliberate answer written down, not a default.

---

**The agent's own "if you only take one":** the rival's Moment (1). It costs one enum variant, one card in `report.toml` and one saved field, and it is the only item here that makes a rival *interrupt* the player rather than wait to be looked up. The Faction window is already a good answer to "tell me about them"; nothing in the game yet answers "they are about to win" without being asked.
