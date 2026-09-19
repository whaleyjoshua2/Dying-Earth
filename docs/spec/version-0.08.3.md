# Dying Earth — version 0.08.3, the asymmetry version: Research each Faction may spend its own way, a Unique Module for every Faction, the Exodus Call, a dearer Tech Tree with two new Techs, a Prospector victory counted in Ducats, and Emigrants renamed

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.08.3](https://github.com/whaleyjoshua2/Dying-Earth/issues/230), and the
pictures and measurements that decided it are in
[`docs/dev-diary/2026-09-18-version-0.08.3/`](../dev-diary/2026-09-18-version-0.08.3/).

**What the version is.** Version 0.08.2 shipped with the Faction balance written into its own **Out
of scope** section — the Custodians winning **37 games of 80** and the Arkwrights **3** — and the
designer's words there were *"we'll deal with it in future versions."* This is that version. Seven
of its changes hand a Faction something only it can do; two impose a cost; the rest is housekeeping
asked for in the same breath.

**What it did to the win column**, 20 seeds across four seatings at the shipped climate cell
(sink 6, step 300), against the 0.08.2 baseline:

| Faction | 0.08.2 | 0.08.3 |
|---|---|---|
| Custodians | **37** | **26** |
| Prospectors | 10 | **30** |
| Arkwrights | **3** | **8** |
| Archivists | 7 | **1** |

The runaway came down by eleven and the thinnest seat rose by five, which is what the version set
out to do. It also **made the Prospectors the new runaway and reduced the Archivists to one win in
eighty**, and both of those are measured consequences with named causes rather than surprises —
§10 and §11 say what they are. The designer, shown the column: **ship and record**.

Collapses fell from **23 of 80 to 15 of 80** — the world bites *less* hard than it did, which is the
one figure moving in a direction nobody asked for. The whole Tech Tree completes in 17 of 20 seeds
at a median turn of **26**, against 79 of 80 at turn 31–36, so it finishes *faster* despite costing
17% more.

**One rule on the designer's list was found already present during grilling and closed without
code**: a shot aid that opens a window and parks a pointer
([#242](https://github.com/whaleyjoshua2/Dying-Earth/issues/242)) — `tech:1`, `factions:1`,
`trade:1`, `victory:1`, `climate:toggle`, `window:<w>x<h>` and `tip:<word>` all already existed.

---

## 1. The Tech Tree gets dearer

*Ticket [#231](https://github.com/whaleyjoshua2/Dying-Earth/issues/231).*

**Coastal Engineering costs 15**, **every rung-2 Tech 32**, **every rung-3 Tech 48**. Rung 1 is left
at 18: a dearer rung 1 slows all four seats identically and moves nobody relative to anybody.

Measured before it was taken: the tree went from **554 Research to 585**, a rise of 5.6%, about 1.8
turns of late-game Research. With the two new Techs of §2 it stands at **20 Techs and 649 Research**,
a rise of 17%. Told that figure, the designer: *"proceed we'll correct any overcorrection later."*

Coastal Engineering stays the cheapest thing in the tree and is still taken by turn 5 to 8.

---

## 2. Two new Techs: off-world Mines pay a tenth more, and a Relay pays double Influence

*Ticket [#232](https://github.com/whaleyjoshua2/Dying-Earth/issues/232).*

**Beneficiation** — Extraction, rung 2, 32 Research, needs Deep Mining. Mine output ×1.1 **wherever
the Mine stands, Antarctica included**: 13 of 16 Mines measured were Antarctic, so "off world" read
literally would have reached 0.15 Mines a game.

**Relay Networks** — Off-world Living, rung 2, 32 Research, needs Expanded Habitats. A Relay adds
**2** Influence to its holder's Allotment instead of 1. The designer moved the +1 Influence **off the
Habitat and onto the Relay**, which the measurement supported: **one Relay was built in forty games**.

**The larger half of this ticket is not the Techs.** No Tech in the game had ever changed what a
computer seat wants to build — a Mine's and a Relay's AI weights were flat and read nothing, so Deep
Mining's ×1.5 and the Extraction Charter's ×1.25 had never once made a seat want a Mine more. Both
weights now read what their Techs are worth. The designer, asked whether to fix Beneficiation alone
or the gap: *"let's fix that one outright."*

A Mine's weight reads the **tech factor** and never the finished yield: the finished yield carries
the slot's own yield, which is at least 1 everywhere, and a weight scaled by it put **329 Mines** on
the board where 16 had stood.

---

## 3. The Tech Tree scrolls, and every box is a tenth smaller

*Tickets [#243](https://github.com/whaleyjoshua2/Dying-Earth/issues/243),
[#247](https://github.com/whaleyjoshua2/Dying-Earth/issues/247),
[#248](https://github.com/whaleyjoshua2/Dying-Earth/issues/248),
[#249](https://github.com/whaleyjoshua2/Dying-Earth/issues/249),
[#250](https://github.com/whaleyjoshua2/Dying-Earth/issues/250).*

A regression from §2, found by the first picture ever taken of the Tech Tree: two new rung-2 cells
took the tree from 672 pixels to 864, and at 1280×800 the **whole Society branch — two Victory gates
among it — sat off the bottom with no scrollbar.**

The tree now **scrolls** inside a screen-bounded window, with a scrollbar only when one is needed,
and **every layout figure and box font is a tenth smaller**, taking it to 774 pixels.

The band order, settled over four tickets, is **Off-world Living, Industry, Extraction, Propulsion,
Society**, and **every rung-3 Tech stands in one column**.

---

## 4. Two explanations go to hovers

*Ticket [#233](https://github.com/whaleyjoshua2/Dying-Earth/issues/233).*

The Blame sentence and the Relations explainer come off the Faction window and onto **hovers on
their headings**, taking about a fifth off the page — it ran to y≈783 and ends at y≈640.

**No marker.** The recommendation was a small `?` beside each heading; the designer rejected it:
*"no other mouse overs have any ? — the convention here is the same, mouseovers are common AF in 4x
games."*

**Only the Faction window's copy of the Blame sentence moved.** The Climate Panel's plural version
and the top bar's Influence hover keep theirs.

**The pair hover names what STANDS between two Factions, never what could be struck.** The Accords
block a few rows below already lists every term; saying it twice, a scroll apart, is how a figure
drifts. The same rule covers rival-to-rival pairs: what stands is visible on the board once it
bites, what they *could* strike is intelligence.

---

## 5. The Emigrant becomes the Pioneer, and Muster becomes Recruit

*Tickets [#234](https://github.com/whaleyjoshua2/Dying-Earth/issues/234) and
[#244](https://github.com/whaleyjoshua2/Dying-Earth/issues/244).*

**Emigrant is now Pioneer. Muster is now Recruit. Steerage is now Coach Class.** The reason was the
word rather than the rule — *"it's more about the word being politically loaded and racked with
connotation"* — which made a third answer available where the ticket had charted two.

**The distinction survives.** Coach Class and the Spaceport are both stated in terms of it, and
*"every Colonist costs their Region twice the population"* would be false without it.

**Stowaway was argued down rather than adopted**: a stowaway hides aboard without permission or
payment, where these people are recruited openly, counted, and carried in ships built for them.
**Coach Class** keeps the travel-class metaphor, which is the honest description of the mechanic.
**Diaspora stays** — *"Diaspora seems fine"*.

Only prose moved: ~41 Rust strings, the Report and tutorial lines and the glossary, while ~190
identifiers, the Report keys, the data table and the **two serialized save fields** keep the old
spelling, so **no save breaks**.

---

## 6. The Research Directive: every Faction may spend its Research its own way

*Ticket [#235](https://github.com/whaleyjoshua2/Dying-Earth/issues/235).*

A **share of a Faction's Research, chosen as a percentage and standing until changed, goes somewhere
other than the shared Tech**:

| Faction | where it goes | rate |
|---|---|---|
| Custodians | the Natural Sink, **permanently** | 0.01 ppm a point |
| Prospectors | Ducats, as income | 0.8 a point |
| Arkwrights | Fuel | 1 per 5 points |
| Archivists | the Archive fund | as before |

**Half for everyone; all of it for the Archivists alone**, whose slider's lower half is greyed and
will not travel below 50% contribution.

**A measurement changed two of the three rates.** Research over a whole game is only 131 to 453 by
Faction, so 0.05 ppm had no safe reading — 0.09 a turn is invisible, or +2.7 ppm permanent is a 45%
enlargement of the sink on the runaway seat — and "10-1" was ambiguous by a hundredfold.

**A slider, not the switch that was recommended**, and the designer's reason is a dependency the map
had missed: with a switch at half, *any* diversion puts a Faction below 85%, so §7's rule would be
binary. The contribution is shown in the header — **Research Directive: 50%**.

**Provisional Findings** takes a **75% threshold** rather than staying binary, read from the
directive declared at the last Income, which preserves its one-turn lag.

---

## 7. The shared pot: under 85% costs a Faction its friends, all of it pays

*Ticket [#236](https://github.com/whaleyjoshua2/Dying-Earth/issues/236).*

A Faction contributing **less than 85%** of its Research to the shared Tech takes **−1** with every
rival; one contributing **all of it** takes **+1**.

**It is a term, not a deed** — *"plus/minus 1 but only for that turn"* — read afresh every settle as
the Blame term is. Nothing is banked, and a Faction that starts contributing again is forgiven the
same turn. As a deed it measured **78 points of damage a seat over a game**, on a scale bottoming at
−10.

**The reward stops at the top of Cordial**, one band above Neutral, bending 0.08.2's
no-Accord-no-rising rule by one step rather than breaking it. A pair already higher by deeds is not
dragged down to it. **It never scars.**

**The measurement said the rule could not work as written.** Every seat went to its directive cap on
turn 2 and never moved, so the penalty would have fired on everyone for 26 turns of 36 and the
reward never. The designer fixed it without touching the rule — *"the ai need to weigh the benefit
of the new tech to which they contributing"* — and turns below the line fell from **[24, 28, 28, 28]
to [0, 2, 0, 28]**.

---

## 8. The Exodus Call

*Ticket [#237](https://github.com/whaleyjoshua2/Dying-Earth/issues/237).*

The Arkwrights only, on a Nation State they control, for the price of a Leapfrog (**50 Ducats**):
**two turns of a doubled muster** — sixteen Pioneers a turn where they otherwise recruit eight.
**Once per Region, ever**, the shape the Strip Permit has had since version 0.05.

**While it runs it suspends Coach Class's double charge.** Measured before it was decided, their
home Region runs from twenty units of population to **one** over a game, because Coach Class charges
them twice a head; an order that doubled only the count would have emptied the country twice as fast
and deepened the very thing capping them at 3 wins of 80. That clause is the order's point rather
than a sweetener.

Unrest is left alone, with *"this should eventually scale by pioneer"* recorded as future work.

---

## 9. Three turns before you may remake a country

*Ticket [#238](https://github.com/whaleyjoshua2/Dying-Earth/issues/238).*

The **Strip Permit**, the **Leapfrog** and the **Exodus Call** each need the Region held **three
whole turns**. The turn of the taking does not count, so a Region taken on turn 10 opens on turn 13
and a Faction's starting Region on turn 4.

**The clock restarts only when the CONTROLLER changes**, not on every write to a Region's control: a
write back to the same seat happens often enough that a per-write reset would deny all three orders
forever. **A Region with no clock recorded passes**, so a save written before this version is
briefly generous rather than silently three orders short.

**The refusal names the turn it opens** — *"China has been yours for less than 3 turns; you may act
there from turn 4"* — rather than saying no.

Measured, flipping the rule off and on over the same twenty seeds: Leapfrogs issued fall **378 to
177**, a little under half. A refused order is not deferred — the Ducats go elsewhere that turn and
the board diverges. The share kept climbs with the length of the game, 20% on the fastest-collapsing
row to 51% on the slowest.

---

## 10. Three more Unique Modules

*Ticket [#239](https://github.com/whaleyjoshua2/Dying-Earth/issues/239).*

Every Faction now has a Unique Facility on Earth **and** a Unique Module off it:

| Faction | Unique Module | replaces | clause |
|---|---|---|---|
| Archivists | **Heliostat** | Solar Array | +1 Energy, **after** the inverse square scaling |
| Prospectors | **Exchange** | Trade Post | +1 Ducat, flat, **after** the ×1.25 |
| Arkwrights | **Chorus** | Relay | +1 Influence to the **Allotment** per 6 Colonists at its own Colony |
| Custodians | Academy | Institute | unchanged |

**The Arkwrights' Unique moved off the Habitat and onto the Relay** at the designer's word, which
dissolved the question the ticket opened with — Coach Class keeps its ×1.5, and nothing hands a
captor room they could not have built.

**A census reshaped the ticket before a line was written.** Over 120 games, **Trade Posts stood zero
times and Relays zero times**. The Trade Post's cause was a defect: it is the only Producer whose
resource is Ducats, and the Producer bonus fires only for a resource the seat is *short* of, which
Ducats structurally cannot be. Both weights now read what the building would make, clamped to the
band the Mine's tech factor runs in. A/B: Trade Posts and Exchanges standing go **8 to 73 with the
win column unmoved**.

**A picture of the build list found a bug no test could.** The rule for what stands on a Space
Station was written out twice as a list of **kinds**, so `built_by` swapped the common kind out and
the list threw the Unique away — the Prospectors lost the Trade Post row without gaining the
Exchange. It is now one method answered by the **job**, with a test.

**Two of the four Unique Modules are never built by a computer seat**, and the closing sweep says so
across all four seatings: the **Heliostat** 0 and the **Academy** 0, against the Exchange at 29–120
and the Chorus at 217 where the Arkwrights lead. The Heliostat's cause is named: a Solar Array is
raised by the Energy-shortage bonus, and the Archivists' Reactor takes 75% off their Energy upkeep,
so they are the one Faction never short of Energy. **Their Unique is a building their own signature
rule stops them wanting.** The Academy's cause is unexamined and predates this version.

---

## 11. The Prospectors hoard Ducats

*Ticket [#240](https://github.com/whaleyjoshua2/Dying-Earth/issues/240).*

The **Venture Capital Fund holds Ducats**, filled by a share of **Ducat income** taken at Income
before the seat can spend a coin of it, and the bar is **2000**. The name and the Faction's glyph
are unchanged — *"keep both"*. The "plus 12 Colonists living off Earth" half is untouched.

**The ticket's own premise was false and the measurement said so first.** It was titled *"a Victory
Condition no computer seat has ever met"*; the Prospectors meet it **outright in 16 of 20 games**
whenever the world lasts to turn 30. The figure behind the claim had been taken at collapse paces
where every game ends at turn 16, and nothing can be hoarded by then. So the change is **thematic,
not remedial**.

**A share of income rather than a relabelled share of output** is the substantive half. Measured
over 120 games, **every seat ends every game holding about five Ducats** — they spend the lot — so
the Fund now competes with Influence bought, Relief paid and repairs made, and the share is a
decision taken every turn.

**The bar was derived, not converted.** 3000 (1000 Materials at the market's 3 Ducats) would have
been wrong, because Ducat income and Materials output are different curves. Ducat income 1993 a
game × the 80% cap × the ~30% the Investment Bank's interest adds ≈ 2073. Measured after: the Fund
ends at **2049 and 2145 against 2000**, where it ended at 1027 and 1026 against 1000, **and the win
column does not move**.

**A Bank now fills the Fund**, inverting ticket #72's test, which said the opposite in as many
words. **Ducats got by selling are still not income** and never reach it.

---

## 12. The Upload's gate

*Tickets [#245](https://github.com/whaleyjoshua2/Dying-Earth/issues/245) and
[#246](https://github.com/whaleyjoshua2/Dying-Earth/issues/246).*

**The Upload needs Closed-Loop Colonies, and Public Science is dropped from its path.** The line the
designer wanted removed turned out not to be the edge it looked like: it ran **Expanded Habitats →
The Upload**, passing *underneath* the Generation Ships box, so removing it lowered the Archivists'
Victory gate rather than tidying a duplicate. Told that: *"remove expanded habitats from the upload
I accept the weakend gate."* Minutes later the gate was rebuilt on a better parent, since the
Archive stands at a Colony off Earth.

**The Archivists' gate went 2 → 1 → 4 in one version**, and it bars the Archive **order** as well as
the win. Gate depths, counted as Techs that must stand:

| Faction | gate | depth | wins of 80 at the 0.08.2 baseline |
|---|---|---|---|
| Custodians | Planetary Stewardship | **2** | **37** |
| Archivists | The Upload | 4 | 7 |
| Arkwrights | Generation Ships | 4 | 3 |
| Prospectors | The Extraction Charter | 4 | 10 |

**The runaway seat has the only shallow gate.** No ticket set out to do that. A test pins all four
depths so the next change cannot move one silently.

---

## 13. Six figures the sweep could not say

*Ticket [#241](https://github.com/whaleyjoshua2/Dying-Earth/issues/241).*

Version 0.08.2 ended by naming three figures as unreported; this version's rules need three more.
All six now come out of the sweep:

- **Ducats made and spent per seat**, spending *measured* rather than inferred — held going into the
  turn, plus the turn's income, less held coming out.
- **Accords struck** over a game, against the number standing at its end. Struck runs 106–129 a
  batch against 24–50 standing, so roughly three quarters are struck and then broken; that was
  invisible before.
- **Aggregation across the four seatings.** `--seatings` runs all four Factions as seat 0 in one
  process and prints one win column of 80.
- **The Research Directive each seat ran**, as the mean share kept back from the shared pot, and the
  turns spent under 85%.
- **Exodus Calls sounded**, and the Pioneers they were worth.
- **The four Unique Modules standing at the end.**

---

## What the closing sweep says

Run as `sweep 20 --balance --seatings --steps=300`; the output is
[`docs/dev-diary/2026-09-18-version-0.08.3/sweeps/final-0.08.3.txt`](../dev-diary/2026-09-18-version-0.08.3/sweeps/final-0.08.3.txt).

**The win column is in the header of this document.** Beside it:

- **The Archivists' fall to 1 win of 80 is explained, not mysterious.** They keep back **96–97%** of
  their Research, because funding the Archive *is* their Victory path, so they sit under the 85%
  contribution line for **516 to 648 turns of a batch** where every other seat sits under it for 0
  to 58. They pay Relations for their own Victory Condition every turn of every game. On top of that
  their Unique Module is never built (§10) and their gate moved from 2 to 4 (§12). Three separate
  tickets landed on one seat and nobody was watching the sum.
- **The Exodus Call fires 177 times and only in one seating** — the one where the Arkwrights are
  seat 0 and are dealt a rich Region. Zero in the other three. That is §8's expectation sharpened:
  it fires only when they start rich, and that seating is also their best, 8 wins of 20.
- **Ducats made and spent are equal to within a few points for every seat in every seating**, which
  is the figure §11's design rests on.
- **Collapses fell, 23 of 80 to 15 of 80.** A rising collapse rate is read here as the Collapse Line
  doing its job; this went the other way, and nothing in the version aimed at it.

**Nothing was re-fitted on this sweep.** The designer, shown the column: *"ship and record."* Five
changes push on the same seats at once, and re-fitting three of them against one batch is how a
version becomes a balance version.
