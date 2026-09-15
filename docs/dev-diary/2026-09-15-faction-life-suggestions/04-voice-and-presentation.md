# Suggestions: a Faction as a character, not a colour

Brainstormed by an Opus agent on 2026-09-15 after reading the glossary, the 0.07.5, 0.07.6 and 0.08.1 specs, `engine/src/report.rs` and the data files. Proposals only; nothing here is decided. The example lines are written to be used rather than to stand in, but they are the agent's words, not the designer's.

---

## 1. The rival's Moment

**What it is.** A Moment that belongs to a rival, not to the player: fired once per Faction when its Victory progress crosses a high bar with both parts moving. It wears the Faction's symbol and colour and reads like every other Moment — one sentence, one number. *Figure:* `9 of 12 uploaded`. *Text:* "The Archivists have read nine of the twelve their Archive wants. It stands complete at Tycho on the Moon, and it is running." A second kind for the Faction's gate Tech standing.

**Why it brings life.** Nothing in the game currently tells the player a rival is about to end it; a Faction that can frighten you is a character, and one that cannot is a colour.

**Size.** Small. Two `MomentKind` arms, a once-per-Faction latch on the Victory progress the engine already computes, four to eight sentences in `report.toml`.

**Risks.** Fires too early and the player learns the alarm is a lie; fires at 90% and it arrives too late to answer.

**Open design questions.** Where the bar sits (three quarters of the Victory Condition, or a fixed distance — nine of twelve, 800 of 1000). Whether it fires once or again on each further step. Whether a Faction gets one when its own gate Tech completes. *The agent recommends* one bar at three quarters, once per Faction, plus a separate one for the gate.

## 2. The rival's word

**What it is.** One sentence of voice at the foot of a rival's paragraph in the Report, in the Faction's colour, **only** where that turn's loudest deed is on a short earned list — a Strip Permit, a Leapfrog, an Upload, a Colony founded, a place taken from the player. Silent otherwise, which is most turns.

> The Prospectors: three years of output in three turns. Nigeria knew the terms before it signed them.
>
> The Custodians: Brazil will burn a tenth of what it burned, for good, and it cost us fifty Ducats. Ask what your own Regions burn.
>
> The Archivists: four more read into the Archive at Tycho. They have left the living count, and nothing that happens on Earth reaches them now.

**Why it brings life.** The deeds paragraph already says what a Faction did; this says why it thinks it was right, which is the whole difference between a log and an argument.

**Size.** Medium, and the cost is writing: four Factions across eight or ten deeds is forty lines. The code is a second `[rival]` table and a "loudest deed" pick the paragraph builder nearly makes already.

**Risks.** This is the one most likely to be skipped — if it appears every turn it becomes furniture within six turns.

**Open design questions.** Which deeds earn a word. Whether a Faction may speak twice in a game about the same deed, or wants a second and third line per deed for variety. Whether the player's own seat gets one. *The agent recommends* five deeds, two lines each per Faction, and no line for the player's own seat.

## 3. The last word

**What it is.** The Game over modal is four ranking lines and a seed. Give each Faction one sentence beside its line, chosen by whether it won, came close, or was nowhere — and by whether the world collapsed.

> The Custodians: we held net Emissions under the Sink for eleven turns. Then the permafrost went, and eleven turns were not enough. Somebody will say it was always going to. It was not.
>
> The Prospectors: one thousand Materials banked and twelve of ours living off Earth. Nobody has yet explained what the alternative was.

**Why it brings life.** It is the screen the player remembers, and the only place the four can be heard without competing with the turn.

**Size.** Small. Twelve to sixteen sentences and a lookup on the outcome the modal already has.

**Risks.** Almost none — it is read once. A losing Faction written as a sore loser would cheapen it.

**Open design questions.** How many states (won / near / nowhere, or those three crossed with Collapse). Whether a Collapse silences everyone or gives all four a line. *The agent recommends* three states plus a Collapse override, because Collapse is a different kind of ending.

## 4. The Faction states its case

**What it is.** A paragraph at the head of the Faction window, under the 64-pixel symbol and above Victory progress, in the Faction's own voice, picked from a handful keyed to where that Faction actually stands — leading, behind, gate not yet standing, Victory near. The rulebook below it stays exactly as it is.

> The Prospectors. There are nine hundred Materials still to bank and twenty-two turns to bank them in. Every Region that goes to somebody else is a mine that never opens. We are not behind. We are early.

**Why it brings life.** The window was built to say everything about one Faction and says everything except what it wants; this is the cheapest place in the game for a Faction to make its case, because the player opened the window on purpose.

**Size.** Small code, medium writing: four Factions by four positions is sixteen paragraphs.

**Risks.** A paragraph that never changes is wallpaper on the second reading; the keying is what keeps it alive.

**Open design questions.** How many positions, and what the blurb on the card does now that a fuller voice stands above it. Whether a rival's page speaks to you or about the board — a rival addressing the player in a window the player opened is a different register from the Report. *The agent recommends* about the board, saving the second person for suggestion 6.

## 5. The voice budget

**What it is.** Not a feature: a rule, written into the spec before any of the above is built. **At most one line of voice reaches the player unasked per turn**, across all four Factions — Report foot and Moment together — and everything else lives behind a hover or in a window the player chose to open. One tick in the Moments corner, `A Faction's word`, switches the unasked kind off entirely, exactly as a Moment kind can be switched off today.

**Why it brings life.** Voice the player learns to skip is worse than no voice, and this is the only decision that makes the other seven safe to take.

**Size.** Small, and negative work: it caps what suggestions 1, 2 and 6 may spend.

**Risks.** Set too tight, the four never say enough to become characters.

**Open design questions.** Whether the budget is one line a turn or two. Whether switching the tick off should also silence the rival's Moment, which is closer to a warning than to voice. *The agent recommends* one line a turn, with the rival's Moment keeping its own tick.

## 6. The Communiqué

**What it is.** A rare modal, drawn as a Moment is drawn, in which one rival addresses the player directly — earned by something on the board, never by a timer. Candidate triggers: the player has taken two of that Faction's places inside four turns; the Faction's Relations toward the player reaches the floor; the player's Blame share passes half. It states a case and makes no demand the rules cannot carry, which is exactly the vessel the roadmapped diplomacy would later speak through.

> The Arkwrights. You have taken India from us, and Indonesia four turns before it. We hold ground for the people standing on it and nothing else; every Region you take is a launch that does not happen. Take a third and we will muster where your Influence cannot reach.

**Why it brings life.** One letter a game from a Faction you have actually wronged does more than thirty turns of paragraphs.

**Size.** Medium. A new Moment-shaped modal, three or four board triggers with latches, four Factions by three triggers of writing.

**Risks.** A threat the rules never carry out teaches the player to ignore the next one.

**Open design questions.** Which triggers, and whether a Communiqué may ever carry a consequence the engine honours (a declared intent to take a named Region next turn) or must stay pure voice until diplomacy lands. Whether the player can answer. *The agent recommends* pure voice for now, with the triggers written so a later reply has somewhere to attach.

## 7. Events that name a Faction

**What it is.** Every Event is faceless, and version 0.03 removed the two that singled a Faction out for being itself. The way back is to target by **deed, not identity**: an Event whose target is "the Faction whose Blame share is largest, and only above a fair quarter", or "the Faction that most recently issued a Strip Permit". The Event then names them in its text and in the Report.

> **Reckoning.** The {faction} are named in every capital that has lost a coast. Unrest rises by 2 in each of the {n} Regions they hold.

**Why it brings life.** It makes the world answer a Faction for what it did, which is the only way the deck can hold an opinion without being unfair.

**Size.** Medium-to-large: new target kinds in the engine, deck balance (the deck is forty cards for thirty-six turns and is never reshuffled), and a sweep to see what it moves.

**Risks.** A deed-targeted Event is a punishment rule wearing the deck's clothes, and it will move the win table.

**Open design questions.** Whether the deck should take an opinion at all. How many such cards, and whether any of them can reward a deed as well as charge for one. Whether they displace existing cards or enlarge the deck. *The agent recommends* two cards at most, one charging and one rewarding, measured before keeping.

## 8. Names beyond the hull

**What it is.** Ships are named; two small things are not. **A motto per Faction**, one line on the Faction card and under the window's symbol — "What we take back, we take back for good" — which costs four sentences and one field. And **named Armies**, drawn from a per-Faction list as Ship names are drawn from theirs, so a Battle Report reads in names rather than in counts.

**Why it brings life.** A Battle Report that says the Ninth Levy was destroyed at Tycho is a thing that happened to somebody.

**Size.** Small for the motto; small-to-medium for Armies (a third list, a name on the Army record, six or seven places that print an Army).

**Risks.** Named Armies add reading to the Battle Report, which is already the densest block in the Report.

**Open design questions.** Whether Armies belonging to a Region rather than a Faction should take the Region's names instead of the holder's. Whether a motto competes with suggestion 4's paragraph. *The agent recommends* taking the motto and holding the Armies until the Battle Report is looked at on its own.

---

**The agent's own "if you only take one":** the rival's Moment. It is the smallest piece of work on this list, it fills the one real gap — the game never tells you a rival is about to end it — and it gives the last third of every game a named antagonist without a single line the player can learn to skip. The rival's word is the natural second, but only once the voice budget in suggestion 5 is settled, because that rule is what keeps the rest from becoming furniture.
