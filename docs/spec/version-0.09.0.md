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

**Measured**: this moved the column further than anything else on the map. The Prospectors fell from
32 wins in eighty to 10 and the Arkwrights rose from none to 17; collapses 42 of 80. Cards drawn
with nowhere to land roughly halved, because with no duplicate copies a spent off-Earth card cannot
come round again. Which Faction the cards favour is the balance version's question.

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
