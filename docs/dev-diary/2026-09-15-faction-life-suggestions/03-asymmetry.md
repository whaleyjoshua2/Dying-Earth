# Suggestions: asymmetry — the four Factions playing genuinely different games

Brainstormed by an Opus agent on 2026-09-15 after reading the glossary, `first-playable.md`, the 0.08.0 spec and the data files. Proposals only; nothing here is decided.

**Where the four stand today, as the agent read them.** Custodians: three signature clauses (Scrubber, Leapfrog, Production Moved), two faction-only orders, a Unique Facility *and* a Unique Module, and no drawback in the multiplier table (output ×1.0, Emissions ×0.75, Research ×1.25, Influence ×1.2). Prospectors: two clauses, three faction-only orders, one Unique Facility. Archivists: one clause plus four Archive orders, one Unique Facility. **Arkwrights: one clause, which is really a bundle of six multipliers, zero faction-only orders, no Unique Module, no station.** They are the thin one, and the agent cites a win table of 42 / 11 / 7 / 6 across 80 games.

---

## 1. Diversion — one Research sink per Faction

**What it is.** Generalize *Fund the Archive* into a system: every Faction has a standing declaration, set by an order and read at Income, sending its own Labs' and Observatories' Research into a sink of its own instead of the shared Tech, each capped as the Archive fund is. Prospectors → Materials into the Venture Capital Fund. Custodians → ppm added to the Natural Sink that turn. Arkwrights → something on the muster or the berth. Archivists keep what they have.

**Why it brings life.** It turns the shared Tech Tree from a thing that happens to you into the central recurring choice of all four seats, and it is the one lever that touches every Faction at once.

**Size.** Medium — the declaration order, the Income branch, the cap and the Faction window row all exist; three new sinks and three AI weights are the new work.

**Risks.** The tree already completes by turn 16 of 36 and diversion slows it further (arguably a fix, per the playtest note); and a Research-to-Sink pipe hands the Custodians, already at 42 wins, exactly what their Stabilization run wants.

**Open design questions.** Should each sink convert at a rate that makes diverting roughly as good as the Tech, or deliberately worse? Is the Custodian sink a second Scrubber by another door, and does that need a different sink entirely? *The agent recommends* building it, but giving the Custodians the weakest conversion of the four until the sweep says otherwise.

## 2. The Exodus Call — an order of the Arkwrights' own

**What it is.** An Orders-phase order on a Region they direct, shaped exactly like the Strip Permit: free, once in a Region's whole life. For three turns the muster limit there is lifted — as many Emigrants as the Region's population can pay — and when it ends that Region carries a permanent scar (Unrest up for good, or a permanent step down in population growth).

**Why it brings life.** It is the missing third leg: the Custodians buy Emissions away, the Prospectors burn a Region for output, and the Arkwrights would burn one for people, which is what they are for.

**Size.** Small — `Order::Leapfrog` and `Order::StripPermit` are the template, down to the once-per-Region check and the Report line.

**Risks.** Steerage already musters eight; lifting the cap on top could flood a Colony faster than Habitats can hold, which is a soft brake, not a hard one.

**Open design questions.** Free or paid in Ducats? Three turns or two? Is the scar Unrest, population growth, or both? *The agent recommends* free and once-per-Region, to mirror the Strip Permit exactly — the symmetry is legible at the table.

## 3. Unique Modules, one per Faction

**What it is.** The off-Earth half of the 0.08.0 system, which today has exactly one entry (the Custodians' Academy). Give the other three the slot: a Prospector Mine, an Arkwright Habitat or Shipyard, an Archivist Observatory, each replacing the common Module at the common price with one clause. Note the spec says **one Unique Facility per Faction is a rule of the system** — so this is the honest route to more, not a second Unique Facility.

**Why it brings life.** Three of the four Factions build an identical Colony today; the Colony is where half the game happens and none of it currently says whose it is.

**Size.** Medium — the `[[module]]` row, the build-list swap, the capture rule and the icon are all precedented by the Academy; three clauses and three AI weights are new.

**Risks.** A Prospector Mine that emits would break "a Module never emits"; a clause that is just "+25% output" is a number, not an identity.

**Open design questions.** Which common Module does each Faction replace, and what is the one clause? *The agent recommends* Prospectors replace the Mine (a share of its Materials goes straight into the Venture Capital Fund, bypassing the banking share); Arkwrights replace the Shipyard; Archivists replace the Observatory.

## 4. A named drawback each

**What it is.** Every Faction gets one thing it plainly cannot do, written as a rule rather than a multiplier below 1. Candidate for the Custodians: **they may not begin an Occupation** — their Armies defend and retake, never conquer. Candidates elsewhere: a Prospector Region's Unrest never falls below a floor; an Arkwright Region permanently loses population growth.

**Why it brings life.** A Faction with an edge and no cost plays the same game as everyone else with better numbers, and the Custodians' 42 wins of 80 say that is where they are.

**Size.** Small — one predicate per Faction, in the order validator.

**Risks.** This is the sharpest balance lever on the list and could over-correct in one step; it also removes options a human player may have been enjoying.

**Open design questions.** One drawback for all four, or only for the Factions measurably ahead? Should a drawback be thematic (Custodians cannot conquer) or economic? *The agent recommends* all four, thematic, one line each; a drawback only the leader has reads as a nerf, and a set of four reads as a design.

## 5. Victory Conditions with milestones that pay

**What it is.** Each Victory Condition gains two or three interim marks that pay once when crossed: the Arkwrights' first Colony on a second Body, the Prospectors' Fund crossing 250 and 500, each four Colonists the Archivists Upload, each Stabilization turn the Custodians hold. The payment is small and in an existing currency — Influence into next turn's Allotment, or Ducats.

**Why it brings life.** The Arkwrights' condition pays nothing until it is complete, and in 80 games no Faction has ever founded a Colony beyond the Moon, so it pays nothing ever. Milestones make an impossible bar into a ladder you can climb visibly.

**Size.** Medium — a milestone table in `victory.toml`, a crossing check in the End phase, and the AI pace tables want re-scaling to read them.

**Risks.** Paying for progress compounds a leader's lead unless the payments are flat and small.

**Open design questions.** What does a milestone pay, and does it pay the same thing to every Faction? Does crossing one show as a Moment? *The agent recommends* a flat Influence payment, identical across the four, so a milestone is a nudge and never a snowball.

## 6. Four different openings

**What it is.** Retire the near-identical start. Each Faction begins holding something that expresses it on turn 1: an Arkwright Colony Ship already in orbit, a Custodian Scrubber already standing, Prospector Materials and a raised Industry Level, Archivist Research already banked. Value held roughly even and stated in `factions.toml`.

**Why it brings life.** Asymmetry the player can press on their first End Turn beats asymmetry they notice on turn eight. There is already accidental asymmetry here — the Arkwrights hold a Spaceport and the Archivists usually a Reactor from turn 1, while the Prospectors and Custodians start with nothing of theirs — and it was accepted knowingly rather than designed.

**Size.** Small — a start-package table and one line per Faction card; `add_start_facility` already exists.

**Risks.** A free Ship or a skipped build turn is worth much more than its Materials, so "roughly even in Materials" will not be even in play.

**Open design questions.** Should the packages be even in value, or deliberately uneven to correct the 42/11/7/6 win table? *The agent recommends* even first, measure, then tilt — one change at a time.

## 7. Faction Ship rules, and the Ark

**What it is.** Ships are the one system with no faction-only content at all. Give the Arkwrights a Ship type of their own — the **Ark**: a Colony Ship, dear and slow, with a Tank large enough to reach the Mars system without a Refuel and capacity to match. Give the Prospectors a Ship rule instead of a hull: their Ships refuel at any Colony of theirs with a working Refinery, not only at a Space Station.

**Why it brings life.** In 80 games nobody has ever settled Mars, Venus, Phobos or Deimos. The Arkwrights' Victory Condition needs three Bodies and the board has never produced two; a hull that can actually cross is the most direct answer.

**Size.** Medium — a fifth `UnitKind`, its row, its Kind Glyph, its name list and an AI weight; the Prospector clause is small on its own.

**Risks.** This overlaps whatever a space-logistics angle would propose; the reason nobody leaves the Moon may be Fuel, transit time, the AI, or all three, and a new hull could fix none of them.

**Open design questions.** Is the far-Body problem a Ship problem or an AI-weights problem? Should the Ark be Arkwright-only or a common hull behind Generation Ships? *The agent recommends* diagnosing before building — if the AI simply never scores a Mars transit, the hull will sit unbuilt as the Carrier and the Bank did.

## 8. Sketch the fifth and sixth now

**What it is.** Write the two missing Faction cards on paper — multipliers, signature rule, Unique Facility, Unique Module, Victory Condition, Victory gate — without building them. The axes the four leave idle are **money** and **force**: a trade Faction whose score is a network of Trade Posts, and a military Faction whose score is held ground and Orbital Control.

**Why it brings life.** It tells you what the four must *not* take. If a money Faction is coming, the Prospectors' Ducats ×1.2 plus the Investment Bank may be sitting on someone else's identity; if a force Faction is coming, nobody else should get a strength multiplier.

**Size.** Small on paper (two cards, no code); large if built.

**Risks.** Designing for Factions that may never ship can freeze decisions about the four that are worth making now.

**Open design questions.** Are the fifth and sixth still money and force? Does a six-seat table still fit fourteen Regions and six Bodies? Does every Faction still need an off-Earth half to its Victory Condition? *The agent recommends* sketching them but not scheduling them, and using the sketches only as a veto on the four.

---

**The agent's own "if you only take one":** give the Arkwrights an order of their own. They are the Faction with one signature rule, no order, no Unique Module and 7 wins in 80, and the Exodus Call costs a day's work because the Strip Permit is its template line for line. It also does something none of the balance levers do — it makes the thinnest seat *feel* like a seat, which is the whole point of the angle.
