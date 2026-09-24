# Dying Earth — version 0.09.0, the production version: Materials and Widgets pulled apart, one unit of population per million, Armies raised from people, Ships sent to a particular orbit, double the Influence to take a place off Earth, cards that ask a question, a summary at the end of the game, and five interface improvements

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.09.0](https://github.com/whaleyjoshua2/Dying-Earth/issues/331), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-09-23-version-0.09.0/`](../dev-diary/2026-09-23-version-0.09.0/).

**What the version is.** The first version since 0.06.0 to reach into the economy, and the first
ever to change what a build costs. Everything made now costs **work** as well as ore: Mines make
Materials, Factories make **Widgets**, and a build completes when the place it is built at has put
enough work into it (§1). A unit of population becomes **one million people** where it was five, and
one Colonist is still one unit (§2), so an **Army raised now costs people** as a Pioneer does (§3).
In space, a Body's orbits are **named places a Ship sits in** rather than one sky: low orbit is the
lane to the ground and a station is reached only from its own orbit, which among other things means
**a player can give a Blockade at all**, for the first time in any version (§4). Taking a Colony or
a station by Influence costs **double**, with a floor under a place nobody has moved into (§5). Half
the Event deck now **asks a question** instead of simply happening, and the turn cannot end until it
is answered (§6). The game ends on a page that says how it went (§7), and five interface
improvements and four adjustments close the list (§8).

---

## 1. Materials and Widgets pulled apart

The authority is [ticket #332](https://github.com/whaleyjoshua2/Dying-Earth/issues/332).

**Widgets are work.** One Widget is one unit of it. They are a **rate, not a stock**: a place makes
so many a turn and applies them that same turn to what is under way there, and whatever is not
applied is lost. They are never carried, traded or banked, and never enter the Stockpile. There is
no cap on how many builds may be under way; a place's own Widgets are the limit on what it builds.

**Four makers, two names**, by the Refinery's precedent of one name in both lists:

| maker | where | cost | Energy | makes | emits |
|---|---|---|---|---|---|
| Factory | Earth Facility | 20 Materials, 4 Widgets | 2 | 4 Widgets a turn | 0.75 |
| Mine | Earth Facility, new | 20 Materials, 4 Widgets | 2 | 4 Materials a turn, times the Region's lean | 0.75 |
| Factory | Module, new | 20 Materials, 4 Widgets | 3 | 4 Widgets a turn, flat, no Body yield | 1.0 on Earth |
| Mine | Module | unchanged | | | |

The Region's Materials lean, Deep Mining and the Strip Permit follow the Materials to the Earth
Mine; Clean Manufacturing follows the Factory. Every Region that starts with a Factory gains a Mine
beside it, so opening Materials are as they were and Widgets are new on top.

**A base per place.** A Region makes a flat **four** Widgets a turn plus **one per Industry Level**
with no Factory at all, so a bare Region builds a one-turn thing in a turn as it always did and a
built-up one a little faster. A Colony or Space Station makes **four** from its Core Module.

**What a build costs.** Materials paid in full at the order, as before, plus a **Widget figure per
row in the data, four per the build turn it used to take**. The flat turn count is retired: a build
completes at the Resolution its accrued Widgets reach its figure. The Faction discounts reach the
Widget figure too, which the designer had always intended.

**The queue at a place is served in order**: the turn's Widgets fill the earliest-ordered build
first and flow on to the next. Nothing is split. An outright buy in Ducats is kept at twice the
Materials and completes at the next Resolution ahead of the queue.

**Production Moved** pairs like with like now: Factory with Factory and Mine with Mine, so a
mothballed Earth Factory doubles Widgets off Earth, which is what the Custodians' signature always
said.

**A place that changes hands** keeps its queue: a build under way finishes for whoever ordered it
unless the new holder cancels it, in which case **its Materials accrue to the new holder**. A
conquest is a prize.

Widgets are drawn as a **cog**, on the top bar as made and applied, on every card as a rate and a
queue, on a hatched tile as `3 of 8`, and on every build button with an estimate at that place's
rate behind its queue.

---

## 2. One unit of population per million people

The authority is [ticket #333](https://github.com/whaleyjoshua2/Dying-Earth/issues/333).

**One unit of population is one million people, and one Colonist is one unit**, so a Colonist is a
million people where it was five million. **Every Colonist count stands as written**: a Habitat
holds four, the Core four, a Colony Ship four, the Off-world Presence bar twelve.

What moved: every Region's population figure times five (Australia 50.0 to India 1,940.0, 7,860 in
all); the two per-unit emission rates divided by five, so the player-facing *per hundred million*
figures are unchanged; the Research population divisor to 5,000. **Figures that are amounts stated
in units were multiplied by five, not divided**, so the rule they express is unchanged: the refugee
Unrest rate, the Report's net floor, and the Scrubber's people per Scrubber. The constant moved out
of the code into the data.

A Pioneer takes **one unit, one million people**, from its Region. The Region card keeps its shape,
`Region population 1454.4 (1.45B)`.

**Measured**: every converted figure moves nothing at all, proved by a control sweep that is
byte-for-byte the baseline. The one designed change, a Pioneer taking a million rather than five,
moves a seating from 16 collapses in twenty to 18.

---

## 3. Armies raised from people

The authority is [ticket #334](https://github.com/whaleyjoshua2/Dying-Earth/issues/334).

A raised Army takes **one unit of its Region's population, one million people**, at the order, on
top of its Materials and its Widgets, and is refused where the Region has not got it. At a Colony it
takes **one Colonist**, refused at fewer than two so the Core Module is never emptied; the Colony
keeps every Module and has no room for another until a Colonist arrives.

The **Standing Army takes nobody**: it is the state's own, sized by its card, and its replenish and
respawn are its own rules. **The people are gone**: a destroyed Army returns nobody, a marching Army
carries nobody home, and no disbanding rule is added.

The Report's raise line names the people taken, as the Pioneer line does, and the Build Army hovers
state them. The sweep counts raises refused for want of people.

---

## 4. Orbits: a Ship sits in one of them

The authority is [ticket #335](https://github.com/whaleyjoshua2/Dying-Earth/issues/335).

**A Body's orbits are low orbit plus one per Orbital Slot** (Earth 5, Moon 2, Mars 3, Phobos 1,
Deimos 1, Venus 3). Every Ship at a Body sits in exactly one of them; there is no longer a Body at
large. A transit **names its destination orbit before leaving**; a new Ship starts in the orbit of
the Shipyard that built it; and moving between orbits at a Body is an order costing **1 Fuel** from
the Ship's own tank, resolved with the transits.

**Low orbit is the lane to the ground**: an Army lands from it, Colonists unload into a ground Colony
from it, a ground Colony is founded and Bombarded from it, and a lift from a Launch Site arrives in
it. **Orbital Control is held in low orbit** and gates the ground as before.

**A station is touched only from its own orbit**: unloading into it, refuelling at it, blockading it,
attacking it, Bombarding it.

**A Battery covers its own orbit alone**: a ground Colony's Battery denies low orbit, so a Colony
that arms itself keeps a fleet off the surface; a station's Battery shields its station rather than
the whole sky. This narrows what version 0.08.8 gave it, deliberately.

**A Battle is fought within one orbit**, so two fights at one Body are two records and two marks. An
Attack fights the orbit the stack sits in, and an Attack on a station that cannot fight back does
nothing beyond the Battle; **Bombard is the second action that breaks it**. Intercept catches only
what arrives into its own orbit. A Blockade shuts the orbit it is given in: a station starves under a
Blockade of its own orbit, and a ground Colony starves while a rival holds Orbital Control outright
and has a stack on Blockade **in low orbit**. Bombard holds the orbit it acts in: a ground Colony is
broken from low orbit with Control there outright, a station from its own orbit with no rival warship
and no rival working Battery in it.

**What this fixed as much as changed**: the Blockade order has demanded a warship sitting in a rival
station's slot since version 0.08.5, and the interface has never named an orbit on any transit, so
**no human player has ever been able to give a Blockade**. They can now.

**Measured**: the war in space went quiet. Over eighty games the computer seats order one Blockade
and open no orbital Battles at all, because a seat keeps about one warship and the ground rule
rightly stations it in low orbit, where no station sits. Carried to the balance version.

---

## 5. Double the Influence to take a place off Earth

The authority is [ticket #336](https://github.com/whaleyjoshua2/Dying-Earth/issues/336).

The threshold figures off Earth double and nothing else does: a Colonist is worth **20** where it was
10, a station's base **40** where it was 20, and a new **base of 40 under every Colony**, since a
station is built with no crew and a ground Colony with none was worth nothing at all. The station
base **replaces** that floor rather than stacking on it:

| place | before | now |
|---|---|---|
| ground Colony, N Colonists | 10N | 40 + 20N |
| Space Station, N Colonists | 20 + 10N | 40 + 20N |

A station is therefore no longer dearer than a ground Colony with the same crew, which it had been
since stations existed. **The challenge margin stays 20**, so a place held by a high Standing is no
dearer than before; the designer took that knowingly.

The computer ranks a rival Colony by the price it would actually pay rather than by how few
Colonists it has.

**Measured, by a counter this ticket added**: over eighty games, 1,305 Regions, 21 ground Colonies
and 5 stations taken by Influence. A take off Earth is **two per cent** of all takes, so the
doubling moved nothing in the computer's game and the column is unchanged. It is a player-facing
price, and a player takes places off Earth far more often than the computer does.

---

## 6. Cards that ask a question

The authority is [ticket #337](https://github.com/whaleyjoshua2/Dying-Earth/issues/337).

The deck held 22 kinds over 40 cards, fourteen kinds carrying eighteen extra copies between them.
**Every extra copy is cut and replaced by a distinct Choice Card**, so the deck is still 40 cards
and **no card in it is a copy of another**: 22 ordinary Events and 18 that ask.

A **Choice Card is drawn at the head of the turn, before orders**, so that a card which binds this
turn's orders can be answered by a player who then gives orders knowing it. An ordinary Event is
still drawn after orders are committed and behaves exactly as it always did. **The turn cannot end
until a human answers**, in the same shape and through the same door as the Tech pick of #105.
**Every seat is asked the same card in the same turn**, the computer seats answering by a rule of
the card's own, so a rival's answer says something true about its board.

A seat the card cannot touch, with no Region held or no Ship in orbit, is **not asked**, and the
Report says so. **A seat that cannot pay the offer is asked all the same**: the offer is closed to it
and refusing is its only move, so a struggling Faction still feels the card.

The eighteen cards are **rows composed from one vocabulary of effects in data**, not eighteen
bespoke rules, reusing the Solar Storm's transit hold, the Drought's output multiplier, the Methane
Burst's next-turn ppm, the Smear's ledgers and a Rich Seam's Discovery. Only the Trading price
override needed new state. Every figure is a field of the card in `events.toml`.

**Measured**: the column barely moved, 5 / 31 / 1 / 0 against 4 / 32 / 0 / 1 before the ticket;
collapses 42 of 80. Cards drawn with nowhere to land roughly halved, because with no duplicate
copies a spent off-Earth card cannot come round again. The computer refuses 28% of what it is asked
where the first build refused 7%, because a seat that cannot pay must now refuse rather than being
skipped, and a quarter of seat-card pairs are still never asked at all.

---

## 7. A chronicle at the end of the game

The authority is [ticket #338](https://github.com/whaleyjoshua2/Dying-Earth/issues/338).

The ticket was charted as a dated narrative of the game. Three sizes were put to the designer with
the lift stated, and they took none of them: what they wanted was **a summary**. No dated record is
built, no per-event call sites, no new prose templates, and the lift fell from medium-to-large to
small, since both charts and the ranking already existed.

**A page of its own**, reached by a **Chronicle** button on the game-over box, which keeps its Title
screen and Quit. While the page stands, neither the board nor the box is drawn. It carries:

1. **The four Factions ranked** as the End phase ranks them, by score, then Colonists off Earth,
   then Colonies held — not in seat order, as the box had listed them since the first playable.
   **The tiebreak is named** on every row it decided, and each Faction gets **one sentence** on how
   it ended: won on its Victory Condition, won because the turns ran out, or lost on the bar it
   missed, naming what held it back.
2. **A table, a row per Faction**: Materials, Fuel, Energy and Ducats at the end; Colonists off
   Earth; Regions held; Colonies and stations held, the two counted together; Research **made over
   the game**; and Blame in ppm with its share. Every figure was already kept by the game.
3. **The population chart and the temperature chart**, reused unchanged, side by side, their time
   axes carrying the in-game date as every chart in this game does.

The page has a `shot:` name of its own, which the game-over box never had.

---

## 8. Five interface improvements and four adjustments

The authority is [ticket #339](https://github.com/whaleyjoshua2/Dying-Earth/issues/339).

The ticket offered the designer nineteen candidates from the written record and asked for two. They
chose **five**, dropped one as already built (Relations as a band, which ticket #221 built two
versions ago), and one more was taken by §7, which now ranks the Factions at the end.

**The odds a player reads are the whole Battle's.** This is the version's largest single finding.
Since Battles existed the figure on an attack button, on a Ship stack's card and in the Battle
Report has been the **first round's share of strength**, honestly labelled but not what a player
wants to know. It has been wrong in both directions: on one board the honest figure is **1%** where
the button said 19.8%, on another **86%** where it said 61%, and a Battle Report line **9%** where
it said 30%. There is no closed form for a melee carrying hit points, disengage rolls, pursuit and
the escort rule, so the figure is **a thousand trials from its own seed**, never touching the game's
dice, so a seeded game is unchanged and the same board always reads the same. A win is **holding the
field**: a unit of yours neither destroyed nor escaped and nobody else's standing, the same test the
Occupation makes. The words moved with the figure everywhere, and since a Battle is fought per orbit
(§4) a Ship stack's card gives a line per orbit rather than one figure for the Body. The record in
the Battle Report moved too, because it was described as what the button quoted.

**A refusal names the rule, not the price.** Every order was tested for affordability before
legality, so ordering something forbidden answered with what it would have cost. Legality is asked
first now and only a legal order is priced. Which orders pass is unchanged, so the computer's
candidates did not move; only the sentence a player reads.

**The Report's march lines name the Army.** Armies have carried names since version 0.08.4 and the
march, landing and rival-deed lines never used them.

**A Relay or an Embassy is an eye.** A working Relay at a Colony or station off Earth, or a working
Embassy in a Region on Earth, lets its holder read a **rival's building-by-building income** at that
place, on the rival's own card, headed by the thing that grants it. No eye, no block.

**The Tech Tree's edges are routed around the boxes they cross** — and measurement showed there are
none to route. With today's twenty Techs and seventeen edges not one crosses a box: the defect
photographed on issue #246 was killed incidentally by #250's every-rung-stacks. The tree is
pixel-identical. What changed is that the property is now held by code, with a test watched red and
a negative control that adds a crossing edge on purpose.

**The four adjustments**, in the designer's own words: the top bar's Influence reads `20 / 20` where
it read `20 of 20`; its temperature reads `+1.7 C` with the heading-to figure moved onto its hover;
the End Turn button gains padding on its left, taken from the column beside it so the cluster keeps
its width; and the four yields on a Found a Colony door sit **beside** the words rather than under
them. That door is recorded in `docs/HANDOFF.md` as built and never looked at by anybody: the first
horizontal version widened the side panel by ninety pixels and ate that much of the map, which a
picture caught.

**Three things the ticket left to the designer**, none of them oversights: the threat line on a held
Region still quotes the first exchange, because that figure is what the computer's attack bar
compares against and moving one without the other makes the amber warning a lie; the stack card's
founding doors are ragged down their right edge now the yields sit beside the words; and an armed
Frigate against three unarmed hulls reads 24%, because holding an orbit means clearing every hull,
which is not the same as taking Orbital Control.

---

## What the closing sweep says

20 seeds across four seatings at the shipped climate cell (sink 6, step 300), filed as
[`docs/dev-diary/2026-09-23-version-0.09.0/sweeps/final-0.09.0.txt`](../dev-diary/2026-09-23-version-0.09.0/sweeps/final-0.09.0.txt),
against the 0.08.8 baseline. **Read the per-Faction totals at the foot of a sweep file, never the
four-figure arrays on each seating's line**: a sweep rotates which Faction sits in seat 0, so those
arrays are by seat. Two figures were first published from that mistake and corrected on tickets
[#332](https://github.com/whaleyjoshua2/Dying-Earth/issues/332) and
[#337](https://github.com/whaleyjoshua2/Dying-Earth/issues/337).

| | 0.08.8 | 0.09.0 |
|---|---|---|
| Custodians | 43 | **5** |
| Prospectors | 22 | **31** |
| Arkwrights | 1 | **1** |
| Archivists | 0 | **0** |
| collapses, of 80 | 14 | **42** |

**Which ticket did it.** Measured after each rule ticket in turn:

| after | Custodians / Prospectors / Arkwrights / Archivists | collapses |
|---|---|---|
| 0.08.8, the baseline | 43 / 22 / 1 / 0 | 14 |
| §1 Widgets | **9 / 35 / 1 / 0** | 35 |
| §4 orbits | 6 / 27 / 4 / 0 | 43 |
| §5 Influence | 4 / 32 / 0 / 1 | 43 |
| §6 choice cards, and the version | 5 / 31 / 1 / 0 | 42 |

**Widgets did all of it.** The Custodians fell from 43 wins in eighty to 9 and the collapse rate
went from 14 to 35 in that one ticket; the three rule tickets after it move the column by a few wins
either way, which is noise at twenty seeds. The cause of the Widgets move was measured rather than
guessed, and it is not the pace of building: it is **Earth's emissions**. Two buildings pollute
where one did, because the new Mine took the old Factory's place and the Factory kept its own, and
the 0.08.8 games ended a hair under the collapse line. The emissions probe that found it is
[`sweeps/breakdown-probe.md`](../dev-diary/2026-09-23-version-0.09.0/ticket-332-widgets/sweeps/breakdown-probe.md)
in that ticket's folder, and eight sweeps beside it show what each figure does. The designer set the
two makers at 0.75 each knowing the rest of the column would go to the balance version.

**The rest of the board**, over the eighty games: 306 Battles opened, 269 Armies built, 92 warships
built, 635 orbit changes and **2 Blockades ordered**; 1,404 places taken by Influence, of which
**1,379 are Regions, 17 ground Colonies and 8 stations**, so a take off Earth is under two per cent
of all takes; and of the choice cards, **1,551 taken, 612 refused and 733 not asked**.

**Two findings carried to the balance version**, both measured here and neither an alarm:

1. **The war in space is quiet.** Two Blockades and no orbital Battles in eighty games. A seat keeps
   about one warship and §4's ground rule rightly stations it in low orbit, where no station sits,
   so it is never beside anything to shut. The dials are the Blockade appetite and how many warships
   a seat wants holding low orbit.
2. **The column belongs to Earth's emissions**, not to any rule of war or trade. Until the two Earth
   makers' emissions are set deliberately, every other balance lever will be read through a collapse
   rate that one figure governs.
