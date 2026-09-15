# Dying Earth — version 0.08.0, the schooling version: a School that moves an Education Level, a Unique Facility for every Faction, Resistance, Relations, and an Archive you have to fill

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.08.0](https://github.com/whaleyjoshua2/Dying-Earth/issues/180), and the
pictures that decided it are in
[`docs/dev-diary/2026-09-13-version-0.08.0/`](../dev-diary/2026-09-13-version-0.08.0/).

**What the version is.** The Education Level stops being a fixed number on a card and becomes
something a player moves, and then five other rules read it. **A School** raises it; **an Institute**
does the same off Earth; **Colonists carry** the schooling of the Region they were mustered in, so a
Colony's figure is the weighted average of who settled it. Schooling then **multiplies a Lab twice**,
**moderates what a large population adds**, and is what **Resistance** reads, which taxes an
outsider's Influence spending rather than the gate. Beside that, **every Faction gains one Unique
Facility** — a building of its own that replaces a common one at the common price — the
**Constabulary** becomes a defensive building, **Relations** begin being kept, and **the Archive**
stops being a monument you stand beside and becomes one you have to fill.

---

## 1. The Unique Facility

*Tickets [#181](https://github.com/whaleyjoshua2/Dying-Earth/issues/181),
[#182](https://github.com/whaleyjoshua2/Dying-Earth/issues/182),
[#183](https://github.com/whaleyjoshua2/Dying-Earth/issues/183),
[#184](https://github.com/whaleyjoshua2/Dying-Earth/issues/184),
[#186](https://github.com/whaleyjoshua2/Dying-Earth/issues/186).*

**A Unique Facility is a building of its own, not a clause on a common one.** It has its own name,
its own icon and its own row on the build list, and it **replaces** the common building on that
Faction's list. A Prospector never builds a plain Bank; they build theirs. The word is **Unique
Facility** on Earth and **Unique Module** off it, because `CONTEXT.md` keeps Facility and Module
apart and the designer's instruction was to keep the distinction the glossary already draws.

**It costs nothing extra**: the same Materials, the same build turns, no Tech, no cap beyond
whatever its own clause names. **One per Faction as a rule of the system**, not an accident of where
this version landed: a Faction is defined by its multipliers, its signature rule, its Victory
Condition and one Unique Facility, and the fifth and sixth Factions are each designed with that slot
to fill.

### Capture: unique in who builds it, not in who benefits

**A Unique Facility is never destroyed when its place changes hands.** It keeps standing and pays
its new holder the same clause it paid its builder. This is the opposite of the **Scrubber** and the
**Archive**, which are destroyed outright on capture; `CONTEXT.md` now says that distinction plainly.

**The mirror does not hold, and that is deliberate.** A **common** building already standing does
not convert when the Faction whose Unique version it is takes the Region. The Prospectors capture
Brazil with three plain Banks in it and those stay three plain Banks; to get interest there they
decommission and rebuild. The bricks are the bricks, and the rule cuts both ways or it is not a rule.

**Only a controller collects.** An occupier pays a Unique Facility's upkeep and draws nothing from
its clause until control transfers — the precedent ticket #69 set for an occupied Research Lab.

### The starting position

**A Faction's start Region's Facilities come up as that Faction's versions** where a version exists.
Every start Region is handed a Launch Site, so the **Arkwrights hold a Spaceport from turn 1**; ten
of the fourteen Regions start with a Power Plant, so the **Archivists usually hold a Reactor**; and
neither the Bank nor the School is in any Region's start Facilities, so the **Prospectors and the
Custodians start with nothing of theirs** and must build. The asymmetry is accepted knowingly.

### The four

**The Investment Bank** (Prospectors) replaces the Bank. It banks **1% of the Venture Capital Fund's
balance** into the Fund each turn at Income, read **before** that turn's banking is added, rounded
down. The Materials are **created**, not drawn from the Stockpile, and they compound from next turn.
**One per Region pays**, however many stand there; there is no Faction-wide cap, so fourteen Regions
with one each compound at 14% a turn. A **floor of +1 Material applies to one building only,
Faction-wide**. It still pays its Ducats in full. In a **non-Prospector's** hands the same share
applies to that Faction's **Ducat income** instead, at a minimum of one Ducat, one per Region,
because no other Faction has a Fund for it to pay into.

**The Spaceport** (Arkwrights) replaces the Launch Site. It pays **+1 Influence for every Emigrant
it lifts off Earth**, which means the two launches: onto a **Ship in orbit**, or onto a **Space
Station of theirs over Earth**. **The sea to Antarctica pays nothing** — it is explicitly not a
launch and Antarctica is explicitly on Earth. It lands as free Allotment paid into **next turn's**,
and it is paid at face value **outside the Allotment formula**, so the Arkwrights' ×0.8 Influence
multiplier never touches it: the 0.8 says they are bad at diplomacy, and this clause says they are
good at moving people. **No cap** — the designer's word was that the muster limit is the brake.
Lifting an Army pays nothing; the Emigrant is what is counted, once per Emigrant however many
Spaceports stand.

**The Reactor** (Archivists) replaces the Power Plant. While one stands and is **online**, the
Archivists pay **75% of the total Energy upkeep of everything they own** — every Facility in their
Regions and every Module at their Colonies and stations — **except the Archive**, which pays its
figure in full. Taken **off the total, rounded down, once**, never per building: upkeep figures are
small whole numbers and most of what a seat owns costs 2 or 3, so per building a 75% rule floors to
a 50% cut on a 2 and a 33% cut on a 3, which would make a Habitat the most Energy-efficient thing
the Archivists could own. **No stacking**: a second Reactor is an ordinary power station.

**The Academy** (Custodians) replaces the **School** on Earth and the **Institute** off it — one
name in both halves. It does everything they do, and pays **+1 Ducat a turn, flat**, wherever it
stands while it is online. Flat rather than scaled by GDP, which would make it a second Bank built
where the money already is rather than where schooling is wanted; flat is also why a captured Academy
pays its captor exactly what it paid its builder, since 1 through the largest output multiplier in
the game floors back to 1.

### The computer players

Each seat is taught **a slight bias** toward its own Unique Facility over the common counterpart.
Kept small on purpose: ticket #41 measured a Ducat-hungry AI over twenty seeds and it cost the
Custodians every win. **The Prospectors are the exception** — their appetite for the Investment Bank
**scales with the balance of the Venture Capital Fund**, which is arithmetically right rather than a
hack, since the building's worth *is* a share of that balance.

---

## 2. The Prospectors' bar moves from 750 to 1000

*Ticket [#182](https://github.com/whaleyjoshua2/Dying-Earth/issues/182).*

Measured first, over 40 games in two seatings: **nobody had ever reached 750**. The highest balance
ever seen was **745**, the median game ended around turn 28 with the Fund in the 400s, and the
computer built **at most one Bank, ever**, in any seed. The Prospectors' 0 wins of 80 followed from
that directly.

The bar rises because uncapped interest at one Bank per Region is worth far more than the +250 the
raise takes away: on the measured trajectory a Bank in each of a median ten Regions pays around 44
Materials a turn by turn 24 and 64 by turn 30, against a seat whose entire net Materials income after
the 80% skim is 4 to 19. **The Investment Bank stops being optional.**

**750 appeared in six places and all six moved**: `factions.toml` (`victory_first.bar` and the card
text), `victory.toml`'s comment, `ai.toml`'s `[pace.prospectors] first` schedule — **rescaled by
1000/750 rather than given a new last number**, since it is what makes the computer set its banking
share every turn — `CONTEXT.md` (the **Prospectors** and **Venture Capital Fund** entries), and the
Fund tooltip in the interface.

---

## 3. The School, the Institute and a moving Education Level

*Tickets [#185](https://github.com/whaleyjoshua2/Dying-Earth/issues/185),
[#189](https://github.com/whaleyjoshua2/Dying-Earth/issues/189).*

**The School** is a Facility, at most one per Region, buildable by any Faction, priced as a Research
Lab at two turns rather than one because it is infrastructure and should not be an instant answer to
a rival's Influence push. While it stands and is online it raises its Region's **Education Level** by
**+0.25 a turn to a ceiling of 2.00**; when it stops, the figure **falls back at the same rate** and
stops at the card's own. The card figures run 0.70 to 1.50 and no maximum had ever been declared, so
the designer set one. A step of 0.25 is the smallest that reliably shows, since a Lab's Research is
floored and a lift of 0.10 crosses no integer at all.

**The Institute** is the same building off Earth: a Module at a Colony or a Space Station, one per
place, at the same step to the same ceiling.

**Colonists carry the schooling of the Region they were mustered in.** A batch takes its Region's
figure **as it stands at the muster**, so a batch mustered after a School has run knows more than one
mustered before it. A Colony's Education Level is the **weighted average** of the settlers who
founded and joined it, and a place with no Institute decays back to that average rather than to a
card figure it does not have.

---

## 4. Schooling moderates what a population adds

*Ticket [#188](https://github.com/whaleyjoshua2/Dying-Earth/issues/188).*

A Research Lab's figure is multiplied by a population factor and, separately, by the Education Level.
The population factor now carries the Education Level too: **1 + (population / 1000) × Education
Level**. Uncapped in both directions, and never below 1, so a badly-schooled Region's people are worth
less than they were but never worth less than no people at all.

**Schooling therefore applies twice to a Lab** — once inside the population factor and once as the
outright multiplier it has always been — and the compounding is the point: it is what makes a School
in a big, badly-schooled Region transformative rather than marginal. The Observatory's per-Colonist
bonus is moderated the same way, so the rule reads the same in both halves of the game.

---

## 5. Resistance

*Ticket [#187](https://github.com/whaleyjoshua2/Dying-Earth/issues/187).*

**A place's schooling takes a bite out of the Influence an OUTSIDER spends there.** Standing gained =
spent ÷ resistance, rounded down. **The controller converts in full.**

A figure of 1.00 has no effect. Below it the resistance scales down to **0.90** at 0.70, the lowest
Education Level on any card; above it, up to **1.10** at 2.00, the School's ceiling. Each side scales
to its own end because the range is asymmetric — 0.30 below the pivot and 1.00 above — and a single
coefficient would leave the −10% floor unreachable by any Region on the board. A Faction with an
Education Level of 2 has +10%, so a rival spending 30 Influence moves their needle by 27.

**It touches neither the threshold nor the challenge margin.** Measured, the **margin** is the binding
figure in 190 of 200 takes of a held Region, so a threshold rule would have missed the contested board
entirely, while a tax on the spending does not care which figure binds.

---

## 6. The Constabulary raises the challenge margin

*Ticket [#190](https://github.com/whaleyjoshua2/Dying-Earth/issues/190).*

**While a Constabulary stands and is online in a Region, the challenge margin there is 25 instead of
20.** For **every challenger, whoever built it**: a police force serves the government of the day, and
a Faction that builds one in a Region it later loses has made its own job harder. **Nothing on a
neutral Region**, which has no margin at all — the price there is the threshold alone. At most one
Constabulary stands in a Region, so no stacking question arises.

Measured over 800 games, **the clause will almost never fire under the computer players**: a
Constabulary stands on 0.5% of Region-turns, only 29 of 9,463 Influence takes had one standing, and a
margin of 25 would have blocked **7** of them — about one take per 114 games. It is kept because it
gives a **player** a reason to build a Constabulary that the computer does not have: today it is
purely an Unrest tool raised in emergencies, and after this it is also a defensive building raised in
a Region you mean to keep. **The sweep is expected to show no movement from this clause at all**,
which is the rule working as decided rather than failing.

A threshold rule was measured instead and would have blocked **0 of the 29**, because on every one the
holder's Standing plus the margin already exceeded the threshold.

**The Region card carries a three-line breakdown** in the designer's own format, ending in the
Resistance conversion, with Blame and Green Consensus joining the threshold line only while they bite.

---

## 7. Relations

*Ticket [#191](https://github.com/whaleyjoshua2/Dying-Earth/issues/191).*

**Every Faction keeps a score for every other: one per ordered pair**, so twelve in a four-seat game.
The scale runs **+10 to −10, starting and neutral at 0**. It falls by **1 for each offending turn** —
a turn in which the offender **spent any Influence on a place the victim holds**, or **opened a Battle
against them** — charged **per turn, not per spend order**. It recovers **+1 every four quiet turns**
and **stops at 0**: it never rises above neutral.

**A place the victim holds** means a Region they control or occupy, a Colony, or a Space Station. **Not
a neutral place**, however hotly contested: two Factions bidding for empty ground are competing, not
crossing each other.

**The +10 half of the scale is reserved and nothing fills it in this version**, so that the scale reads
naturally and 0 is genuinely neutral. Letting peace accrue goodwill was measured and rejected: half of
all ordered pairs never interact at all in a whole game, so the goodwill would mostly be between
Factions on opposite sides of the board who have never met.

**In version 0.08.0 the score does nothing mechanical.** It is read, not spent: no rule reads it and
**the computer players do not read it**. Giving the computer a grudge would re-open the denial
multiplier ticket #50 deliberately removed, and would move every figure in the sweep at the same moment
six other rules are moving them. It is shown as a **four-by-four grid on the Victory window** and as a
sentence in a rival's Report paragraph on a turn their view of you changed.

---

## 8. The Archive's two gates and the Upload

*Tickets [#192](https://github.com/whaleyjoshua2/Dying-Earth/issues/192),
[#199](https://github.com/whaleyjoshua2/Dying-Earth/issues/199).*

**Ordering the Archive wants two things, both checked once, when the order is placed**; neither the
three-turn build nor the standing Module cares afterwards.

**The Archivists' gate Tech, The Upload, must stand.** It is named first of the two refusals, because
it is the one still true after the other is solved: the four Colonists arrive at a median turn 11 and
The Upload at a median 15. The gate Tech already gated the **win** (ticket #84); measured, it arrived
one turn before the Archivists were ready anyway, so it gated nothing. Gating the **order** is what
moves them.

**And four Colonists must live at the place.**

**A new requirement: twelve Colonists must be Uploaded**, and the Archive may only draw from the
population of the place it stands at. **Uploading is an order, and it is free** — an order because it
is irreversible and this game asks before anything irreversible, free because the 80 Research and the
Energy upkeep are already the monument's price. It needs the Archive **complete** first. **It may be
done in batches**: four at a time from a Core Module alone is enough, three times over, which keeps
the Habitat off the Archivists' critical path. **An uploaded Colonist leaves the living population.**

**The Victory Condition's second part becomes "twelve Colonists uploaded"**, replacing "twelve
Colonists living at the Archive's Colony". The count is **monotonic**, and it removes the odd case the
old wording allowed, where a Faction won by having twelve people standing *next to* a finished Archive
rather than inside it.

The ticket was charted as twelve at the order, and measurement forced the redesign: the Archive is
ordered on **turn 1, at a station with nobody on it, in 80 of 80 games**, always Axiom over Earth,
which is founded bare. Under a twelve gate it was orderable in **29 games of 80** at a median turn 17,
and only 17 of those had the three turns the Module needs left.

**Banking into the Archive fund before the Tech is unchanged**: it can still start on turn 1, capped
at a quarter of the 80 until the Module stands. It is the one thing the Archivists can do while they
wait, and stopping it would delay the Archive twice over. It has a useful side effect: the fund fills
to its cap in about three turns, after which their Labs feed the shared tree again, so they start
leading the Research race and can pick their own gate -- which the computer already does when it
leads. The seat chooses between its monument and its tree, which is a better game than diverting
everything and letting rivals do the science, as it did before this gate.

**The computer is deliberately not taught this gate.** Every AI candidate goes through `check_order`
before it is chosen and through `check_order_legality` before it can reserve Materials, so a refused
Archive is skipped at no cost and cannot freeze the seat's build programme. A guard in the AI was
written first and removed when its false state could not be constructed -- the signal that it was
claiming to prevent something the validator already prevents. Ticket #192's Colonist gate is
different and stays: it chooses **which** Colony to name, which no validator can do.

**Why this gate and not a larger one.** The whole seventeen-Tech tree completes by **turn 16 of 36**,
so a rung-3 Tech is a mid-game milestone rather than a late one, and the Archivists contribute **0
Research to their own gate** -- rivals finish it for them while every Lab goes to the Archive fund.
That larger finding is **not** addressed in this version: changing the Research pace in the same
version that changed what a Lab makes, what a population adds and when the Archive may be begun would
make the sweep unreadable. It is
[its own ticket](https://github.com/whaleyjoshua2/Dying-Earth/issues/201).

---

## 9. Two doors the rules already had

*Tickets [#193](https://github.com/whaleyjoshua2/Dying-Earth/issues/193),
[#194](https://github.com/whaleyjoshua2/Dying-Earth/issues/194).*

**Every Colony Ship of the player's at Earth with room left gets a button in the Region card's
Emigrants block**, beside the sea and station buttons, sending min(Emigrants waiting, room to
capacity). It **never offers the crowded places**: above +1.8 a Ship lifting at Earth may take
Colonists beyond its capacity and each of those may die on arrival, and a risk that drowns people wants
the sentence explaining it beside the button — which is on the Ship's card, where the crowding decision
belongs. Both doors write the same `Load` order.

**The Research figure and the Research race bar both open the Tech Tree**, and they **open** it rather
than toggling it: a player who clicks a figure is asking to see what is behind it. The `Tech Tree (T)`
button keeps toggling.

**The Tech Tree's legend** was rendering in the gap between the Industry and Propulsion bands, lying
across the connector lines. The tree paints every box at absolute coordinates, but each **Pick** button
is placed with `ui.put()`, which advances the layout cursor — so the legend started from wherever the
last Pick button landed, inside the tree. That is why it read as wonky *when picking a Tech*: the set of
boxes carrying a Pick button is exactly what picking changes. It is now laid out after the tree's
allocated rect. **The Tech Tree window also gets a default position clear of the Climate Panel**, which
was covering its left third and four of its five legend swatches on the Earth view.

---

## 10. Vocabulary and housekeeping

*Ticket [#195](https://github.com/whaleyjoshua2/Dying-Earth/issues/195).*

`CONTEXT.md` gains **Unique Facility**, **Unique Module**, **School**, **Institute**, **Academy**,
**Resistance**, **Relations**, **Upload** and **opens_on**, and amends **Faction**, **Facility**,
**Bank**, **Education Level**, **Constabulary**, **Influence**, **Standing**, **Threshold**,
**Research**, **The Archive**, **Archivists**, **Prospectors**, **Victory Condition** and **Venture
Capital Fund**.

Three stale statements were corrected: `CONTEXT.md` said there were **twelve** Regions in both the
**Region** and the **Nation** entries, where there have been **fourteen** since ticket #125 cut Japan
out of East Asia and Saudi Arabia out of the Middle East in version 0.07.2; and `factions.toml`'s
Arkwright note read **"Population 19.4"**, the pre-0.07.3 unit, where India is **388** in today's units
of five million people.

**`home` is renamed `opens_on`** in `factions.toml`. It has one reader and its only job is pointing the
start globe's camera, which the word `home` did not say — it read as a starting position, which it has
never been.
