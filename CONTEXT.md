# Dying Earth

A single-player, turn-based strategy game about colonizing the solar system before ecological collapse overtakes Earth. It should feel like a strategy board game rather than an action game.

## Language

**First Playable**:
The small version of the game: twelve turns and two Factions as first built, twenty-four turns since version 0.02, four Factions since version 0.05 and thirty-six turns of two months since version 0.05.5. Its specification is `docs/spec/first-playable.md`, amended by `docs/spec/version-0.02.md`, `docs/spec/version-0.03.md`, `docs/spec/version-0.04.md`, `docs/spec/version-0.05.md`, `docs/spec/version-0.05.5.md`, `docs/spec/version-0.06.0.md`, `docs/spec/version-0.07.0.md`, `docs/spec/version-0.07.1.md`, `docs/spec/version-0.07.2.md`, `docs/spec/version-0.07.3.md`, `docs/spec/version-0.07.4.md`, `docs/spec/version-0.07.5.md`, `docs/spec/version-0.07.6.md` and `docs/spec/version-0.08.0.md`.
_Avoid_: slice, MVP, demo, v1, prototype

**Faction**:
A competing power with its own multipliers, signature rule, Victory Condition and -- since version 0.08.0 -- one Unique Facility. A Faction also keeps a Relations score for every other. The finished game has six; the First Playable has four since version 0.05, the Custodians, the Prospectors, the Arkwrights and the Archivists, and all four sit at every table. Since version 0.07.5 each also has a **symbol**, drawn in the Faction's own colour. Version 0.07.5 confined it to its card on the Faction screen; since version 0.08.1 it is worn wherever something belongs to somebody -- at the head of the **Faction window** at 64 pixels, on a **Colony card's** heading where a Region card wears its Nation's flag, and on every **Ship** row -- a neutral place wearing none. These are the only callers that pick an icon's colour rather than reading it from the fill rule, since a Faction symbol is exactly a statement of whose.
_Avoid_: side, team, empire

**Custodians**:
The Faction that colonizes the solar system while limiting ecological damage to Earth. Their signature rules are the Scrubber, Leapfrog and, since version 0.06.0, Production Moved, and they win only if Earth is still habitable.
_Avoid_: Stewards (the retired name), environmentalists, greens, moderates, eco-terrorists

**Prospectors**:
The Faction that maximizes resource extraction without regard for ecological cost. Their signature rule is Cheap Industry and the Strip Permit, their Unique Facility is the Investment Bank, and since version 0.05.5 they win on a hoard: **2500 Ducats** in their Venture Capital Fund since version 0.08.4 (2000 in 0.08.3), and expansion. It was 1000 Materials before that.
_Avoid_: Extractors (the retired name), capitalists, industrialists, exploiters

**Arkwrights**:
The Faction that exists to get people off Earth and spread them as widely as it can. Their signature rule is Coach Class, and they win on Diaspora. Since version 0.08.6 they open with two Pioneers already waiting in their start Region -- a gift outside Coach Class that took no population, the mirror of the two Colonists the other three Factions have aboard the station the Arkwrights do not have -- who wait there for the first Ship.
_Avoid_: settlers, arks, exodus, nomads

**Archivists**:
The Faction that means to save what humanity knows, and as many of its people as it can, in one place off Earth. Their signature rule is Provisional Findings, their Unique Facility is the Reactor, and they win by completing the Archive and Uploading twelve Colonists into it. Since version 0.08.0 they may not begin the Archive until The Upload, their own gate Tech, stands -- so their Labs' Research is a choice between the monument and the tree that opens its door.
_Avoid_: scholars, librarians, scientists, the Academy

**Exodus Call**:
The Arkwrights' signature order, since version 0.08.3: on a Nation State they control, for the price of a Leapfrog, **two turns of a doubled muster** -- sixteen **Pioneers** a turn where they otherwise recruit eight. **Once per Region, ever**, the shape the **Strip Permit** has had since version 0.05, and like both older orders it needs **Three Turns Held**.
While it runs it also **suspends Coach Class's double charge**: those sixteen cost the Region the ordinary population a Pioneer costs anybody else, not the Arkwrights' double. That clause is the order's point rather than a sweetener. Measured before it was decided, their home Region runs from twenty units of population to **one** over a game as it is, so an order that doubled only the count would have emptied the country twice as fast -- deepening the very thing that leaves them the thinnest seat at the table.
_Avoid_: mass evacuation, the call (alone), lifeboat, rapture

**Coach Class**:
The Arkwrights' signature rule: their Colony Ships carry twice the Colonists and cost less to build, and they recruit eight Pioneers a turn where others recruit four, but every Pioneer costs their Region twice the population.
Called **Steerage** until version 0.08.3, and renamed for the same reason the Emigrant became the **Pioneer**: steerage is literally the cheapest class of passage on an emigrant ship, and this entry's own avoid list had been warning off *cattle class* and *packing them in* since it was written. **Coach Class** keeps the metaphor that is honest about the rule -- bulk passage at low cost -- and drops the history. The designer rejected **Stowaway**, which was considered and argued down: a stowaway hides aboard without permission or payment, where the Arkwrights' people are recruited openly, counted, and carried in ships built for the purpose.
_Avoid_: steerage, mass transit, cattle class, overcrowding, packing them in

**Diaspora**:
The Arkwrights' Victory Condition: thirty Colonists living off Earth, spread over at least three Bodies with at least four Colonists on each. Antarctica counts toward neither part; since version 0.06.0 Colonists on a station over Earth count toward the thirty, but Earth is never one of the three Bodies.
_Avoid_: spread, expansion, exodus, colonization score

**Project**:
Retired in version 0.05.5. The word named a construction raised in stages, each paid in Research and Materials; the Archive was the only one, and it is a Module now.
_Avoid_: project, megaproject, wonder, great work, campaign, stage

**The Archive**:
A Module only the Archivists build, at one Colony off Earth and at most one per Faction, three turns to raise from its own button. Since version 0.08.0 it may be ORDERED only when two things hold, both checked once at the order and never again: the Archivists' gate Tech, **The Upload**, must stand, and **four Colonists** must live at the place. Standing, it still wants its Research, paid into the Archive fund at any pace; complete, with every point paid, it takes a great deal of Energy to keep running, and only then may Colonists be Uploaded into it. It is destroyed outright if its Colony changes hands, and the fund is kept.
_Avoid_: library, vault, monument, database, stage

**Fund the Archive**:
The Archivists' standing declaration that the Research their own Labs make goes into their Archive fund instead of the shared Tech, where it counts nothing toward the Research Lead. Since version 0.07.0 it is set by an order and read at the next Income, before a point of Research reaches the Tech, and it holds until it is set again; the Research is never paid to the Tech and taken back. Until the Archive stands the fund holds only a quarter of what the Archive requires; what the fund has no room for goes on to the shared Tech, and at the cap the declaration is refused.
_Avoid_: donate, invest, bank research, save up

**Provisional Findings**:
The Archivists' signature rule: they already have half the effect of the Tech under research, so long as their Research went to the shared Tech last turn rather than to the Archive. Since version 0.07.6 the Tech it reads is the one standing at the **head of the turn**, not whatever is picked this instant, so a Research Lead changing a provisional pick cannot re-price orders already placed.
_Avoid_: early access, preview, partial tech, head start

**Climate Model**:
The small model of global warming the game runs: Emissions add to the CO2 Stock, the Temperature follows the stock with a lag, and the Temperature acts on population and Events. It moves as a consequence of player and faction choices, never on a fixed schedule.
_Avoid_: warming track, doom clock, countdown, timer, disaster meter

**Body**:
A place in the solar system that can hold a Colony or a Space Station. Six since version 0.06.0: Earth (its Colony Slots are Antarctica's, shut under the ice until +1.6 C), the Moon, Mars, Phobos, Deimos and Venus, a Body of orbits only, with no Colony Slots and three Orbital Slots. A Body may be another's satellite, which sets how far apart they are. Since version 0.09.1 every Body other than Earth carries a **First to a Body** prize for the Faction that settles it before anybody else, larger the further out it lies.
_Avoid_: planet, world, site, location, node

**First to a Body**:
The prize, since version 0.09.1, for founding the first ground Colony a Body has ever carried. It is two things at once: a **windfall** of Influence, paid once and spendable from the turn after the landing -- the Moon 5, Mars 15, Phobos and Deimos 20 -- and a **standing +1** to the founder's Allotment, paid every turn the founder still directs that Colony. Both are paid at face value, outside the Faction multiplier, so the Arkwrights' weakness at diplomacy does not shave a prize for reaching a new world. A Body's first is claimed once and for good, by whoever lands first; the Body's other Colony Slots stay open to everybody. Antarctica claims nothing, being on Earth; a Space Station claims nothing and leaves the ground below it unclaimed; and Venus can never be claimed at all, having no ground to land on. Two Factions landing at one Body on the same turn are parted as a contested slot is: the greater fleet in orbit takes it, and only seats level on strength draw lots.
_Avoid_: first blood, land grab, colonization bonus, discovery bonus, claim

**Upload**:
Reading Colonists at the Archive's own place into it, from version 0.08.0. An order, because it is irreversible, and free, because the Archive's Research and Energy are already its price; it needs the Archive complete, it draws only from the people living where the Archive stands, and it may be done in batches. An uploaded Colonist LEAVES THE LIVING POPULATION -- the place shrinks as it uploads -- and is counted for the Archivists' Victory Condition for good.
_Avoid_: digitize, archive (that is the Module), store, ascend, sacrifice

**Colony**:
A permanent settlement a Faction holds on a Body, founded when a Colony Ship unloads Colonists into a free Colony Slot. It takes the name of its slot ("Tycho on the Moon"). None exist when the game starts. Since version 0.09.1 the first one a Body has ever carried claims that Body's **First to a Body** prize for the Faction that founded it. Since version 0.07.0 it holds three Modules free and one more for every Colonist living there: people gate industry, as Size and Industry Level gate a Region's build slots. The Archive is exempt; a mothballed Module keeps its slot and one building reserves one; a cap fallen below what already stands destroys nothing and simply leaves no room.
_Avoid_: base, outpost, settlement

**Colony Slot**:
One of a fixed number of places on a Body where a Colony can be founded, shared by all Factions. Since version 0.04 each is a real geological place, drawn at its position on the Surface Map and giving its Colony its name. Since version 0.05 each also has its own four yields, drawn when the game starts and never far from its Body's, so no two places on a world are equally worth settling; a free slot shows what a Colony there would get. The four are Materials, Energy, Fuel and, since version 0.07.3, Research: the fourth was the Habitat yield, which set how many Colonists a Habitat there held, until the designer traded it for one that multiplies an Observatory. A landing Colony Ship is founded into a chosen one. Since version 0.09.1 only the FIRST Colony on a Body is worth a prize; the Body's other slots stay open to everybody on the same terms as before. Earth's three, in Antarctica, are shut under the ice until the Temperature has stood at +1.6 C in a Climate phase; once open they stay open.
_Avoid_: site, plot, capacity

**Orbital Slot**:
One of a fixed number of places in orbit around a Body where a Space Station can be built, shared by all Factions, each with a station's name ready for it. Since version 0.09.0 it is also one of a Body's **orbits**, and a Ship sits in it: to touch the station standing there -- to unload into it, refuel at it, blockade it, attack it or Bombard it -- a Ship must be in its orbit and nowhere else. Since version 0.07.3 each is drawn as its own ring round the globe on the Body Surface Map; since version 0.07.4 the rings lie in the globe's own frame on planes of their own, leaning thirty to sixty degrees from the equator with a heading each, so they turn with a drag and cross rather than stack, an empty slot's ring solid grey, and a station's glyph travels its ring, one revolution in about ninety seconds, its name riding beneath it, clickable as it goes.
_Avoid_: dock, berth, orbit

**Low orbit**:
The orbit above a Body's ground, since version 0.09.0, and one of its orbits beside the Orbital Slots. It is the lane to the surface: an Army lands from it, Colonists unload into a ground Colony from it, a ground Colony is founded and Bombarded from it, and a lift from a Launch Site arrives in it. **Orbital Control is held in low orbit**, so it is what a fleet takes to shut a rival off the ground. Before 0.09.0 a Ship not sitting in an Orbital Slot was said to be at the Body at large, which is the same place under a name that said nothing; now it is named, drawn as its own ring innermost on the Body Surface Map, and chosen like any other orbit. A Ship with no orbit named arrives in it.
_Avoid_: the Body at large, parking orbit, the surface, low earth orbit

**Space Station**:
A Colony in orbit, built for Materials into an Orbital Slot with no crew, holding only a Shipyard, Habitats, since version 0.06.0 Observatories and Solar Arrays, a Trade Post and an Institute, since version 0.08.8 Batteries, and since version 0.09.0 the Factory Module, its Widget maker, without which a station's Core alone built everything in it, and a Shipyard took two turns. Since version 0.07.5 it stands with a Core Module, so it holds four people from the turn it is built and can be settled at once; until then it was founded bare and could hold nobody. Influence, Occupation and Battles work on it as on a Colony: since version 0.09.0 its threshold is 40 plus 20 a Colonist, exactly a ground Colony's, where from version 0.04 until then it was dearer by its own base (20, plus 10 a Colonist). Three Factions start with one over Earth (the Custodians the ISS, the Prospectors Tiangong, the Archivists Axiom), and since version 0.08.6 with **two Colonists already aboard**, from nowhere, so a starting station has two Module slots free from turn one where the bare one it was until then had none; the Arkwrights start with none, and build theirs at half price. Since version 0.06.0 a station over Earth is off Earth and its Colonists count for Off-world Presence; it may **no longer hold the Archive**, which since version 0.08.1 must stand on another Body altogether. A station is also the one place a Ship of its Faction can refuel, and since version 0.09.0 only a Ship in its own orbit. It reads the Colony's Module cap since version 0.07.0, so one founded bare holds three and grows only as its people arrive. A station is not a settling: since version 0.09.1 it claims no Body's First to a Body prize, and one standing over a world leaves the ground below it unclaimed.

**Tank**:
The Fuel a Ship carries, since version 0.06.0: a figure per Ship type, filled at the Shipyard for Fuel paid at the build, spent by transits and, since version 0.09.0, one Fuel by a change of orbit at a Body, and refilled only by a Refuel. A Ship whose Tank cannot pay any leg from where it stands, with no Space Station of its Faction there, is stranded until one is built in orbit there; a Tank at nought with a station of its own in another orbit is stranded too, since the change of orbit that would reach it costs Fuel. Since version 0.09.1 a Battle in orbit also spends it: every hull named in one pays two Fuel, and the same two Fuel are the bar a warship must hold to hold Orbital Control, to blockade, to intercept or to fight at its full strength.
_Avoid_: fuel tank, propellant, range, fuel level

**Build Where You Dig**:
The rule, since version 0.06.0, that a Module built at a Colony with a working Mine costs less: three quarters of its price with one Mine, three fifths with two or more, on top of the Faction's own discount and never below half the row. The Archive takes it; a Ship built at a Shipyard there and a Space Station built into orbit do not. A mothballed Mine, or one still building, counts for nothing.
_Avoid_: in-situ discount, local build bonus, mining discount

**Refuel**:
The Orders-phase order that fills a Ship's Tank from the Stockpile, as far as the Stockpile can pay, in the orbit of a Space Station its Faction holds or, since version 0.08.8, one a partner holds under a Refuel Accord -- since version 0.09.0 the Ship must be in that station's own orbit, where before any orbit at the Body would do: the Fuel is always the refueller's own Stockpile's, and a partner's station is only where it is drawn. A station blockaded against its holder fuels nobody. A Ship with no such station and no leg its tank can pay is stranded.
_Avoid_: resupply, top up, tanker
_Avoid_: base, platform, orbital, outpost

**Trading window**:
Where Ducats buy Influence, Materials, Fuel and Energy, spendable in the same turn's orders, and where Materials and Fuel sell back at half. Since version 0.08.2 the price of each of the three goods MOVES with what the whole table bought and sold: a card figure is the middle of a narrow band, enough net buying in a turn raises it a step and enough selling lowers it, it sticks at the band's edge rather than the window refusing, and it comes home one step only on a turn nobody traded that good at all. The window says the price and which way it moved; how much the table bought is not shown. A building can also be bought outright for Ducats from its own build button, at twice its Materials cost.
_Avoid_: market, shop, exchange, store

**Body Surface Map**:
The 3D view of one Body's surface, entered by clicking that Body on the Solar System Map. Every Body has one.
_Avoid_: planet view, ground view, zoomed view

**Earth Map**:
Earth's Body Surface Map, divided into Regions, where Earth-side building, Army orders and the Climate Panel are seen and acted on.
_Avoid_: home view, globe view, terrestrial map

**Solar System Map**:
The 3D view of the solar system, where Bodies, Ship stacks, transits and Orbital Control are seen and acted on. Transits, launches, Ship Stances and attacks are ordered here and nowhere else. Since version 0.05 the sky it draws is the real one: Earth and Mars stand on their rings where they truly stand in the month the turn is, so the distance between them, and the Launch Window, are things the player can see. The rings are drawn to be legible rather than to scale; since version 0.07.4 Venus's ring is at 1.7 and Earth's at 3.8 (Mars at 6.0), spaced so the two inner planets and their station rings never overlap at a conjunction, and a label stands above a planet but hangs below Venus and the satellites so no two ever meet.
_Avoid_: space view, orbital map, star map

**Launch Window**:
The months in which the flight between the Earth system and the Mars system is cheap, because the two worlds stand where a minimum-energy transfer wants them. On the window a crossing takes the shortest flight there is and costs the Fuel on the card; away from it it takes longer and costs more, the further off the dearer, up to a limit. Since version 0.06.0 there are two windows in the game, Mars's and Venus's, each on its own sky; a hop inside the Earth system or inside the Mars system does not have one, and no leg runs between Venus and the Mars system.
_Avoid_: transfer window, launch period, alignment, conjunction

**Event**:
An unplanned occurrence drawn from the Event Deck and taking effect during Resolution — a solar storm, an equipment failure, a discovery. It can never directly break a Victory Condition. An ordinary Event is drawn after orders are committed and simply happens; since version 0.09.0 nearly half the deck is **Choice Cards**, which are drawn before orders instead and ask.
_Avoid_: incident, crisis, card

**Choice Card**:
An Event that asks a question rather than simply happening, since version 0.09.0: it carries two sides, one for taking what it offers and one for refusing, and each side costs something different. It is drawn **at the start of the turn, before orders**, so that a card which binds this turn's orders (*hold every Ship in orbit this turn*) can be answered by a player who then gives orders knowing it; an ordinary Event is still drawn after orders are committed. **Every seat is asked the same card in the same turn**, the computer seats answering by a rule of the card's own, so a rival's answer says something true about their board. **The turn cannot end until a human answers**, exactly as it cannot end while a Research Lead owes the table a Tech. A seat the card cannot touch (no state held, no Ship in orbit, no Refinery standing) is not asked at all, and the Report says so rather than forcing a refusal on a Faction for being small. A seat that simply **cannot pay** the offer is a different case and is asked all the same, at the designer's word during the build: the take side is shut to it and refusing is its only move, so a struggling Faction still feels the card. Eighteen of them replaced the eighteen duplicate copies the deck used to carry.
_Avoid_: dilemma, decision card, prompt, event choice

**Event Deck**:
The deck a card may be drawn from each turn: forty cards since version 0.05.5 for a game of thirty-six turns (twenty-eight before). It held twenty-two Events in one to three copies each until version 0.09.0, when every duplicate copy was cut and eighteen **Choice Cards** took their places, so the deck is still forty cards but **no card in it is a copy of another**: twenty-two ordinary Events and eighteen that ask. Since version 0.08.4 the cards that can only land off Earth -- Grid Failure, Reactor Leak, Dust Storm, Moonquake, Helium-3 Vein, Rich Seam and Ice Deposit -- are not dealt at the start: they are shuffled into what remains of the deck on turn 12, once. There were twelve of them while those seven kinds carried copies; since version 0.09.0 cut every copy there are seven, and the deck is dealt thirty-three cards rather than twenty-eight. It is never otherwise reshuffled, and a card drawn with nowhere to land is spent. Whether a card is drawn at all is the Draw Chance.
_Avoid_: event pool, random table, encounter deck, calm card (retired in version 0.02)

**Draw Chance**:
The chance each turn that a card is drawn from the Event Deck: half at +1.2 C, rising a little for every full fifth of a degree the Temperature stands above it. It replaced the Calm Cards of the First Playable, so the danger in a turn is a percentage rather than a count of blanks.
_Avoid_: event probability, calm cards, event rate

**Tech**:
An advance on the Tech Tree that changes an output, a capacity, an upkeep, a Ship strength, an Influence cost or how much a source emits. When a Tech completes, every Faction has it.
_Avoid_: research, upgrade, invention

**opens_on**:
The Region a Faction's card names for the start screen's globe to face. It has one reader and one job: pointing the camera. It is not a starting position -- any Region may be chosen -- which is why it stopped being called `home` in version 0.08.0.
_Avoid_: home, capital, start state, homeland

**Tech Tree**:
The single tree of Techs shared by all Factions, eighteen of them since version 0.08.1 (seventeen from version 0.06.0), in five branches: Industry, Propulsion, Off-world Living, Extraction, Society. A branch may hold more than one Tech on a rung; since version 0.07.4 two such Techs stack in a taller branch row on every rung but the last, where they sit side by side, so every column is one box wide. One Tech is under research at a time, worldwide. Costs run 18, 30 and 45 by rung since version 0.08.1, Coastal Engineering at 14 being the one exception. Four of the eighteen are Victory gates, one per Faction on rung 3: each is a Tech for everyone, and its Faction cannot win until it stands. Since version 0.07.6 a box chosen this turn but not yet committed wears a paler amber than the settled one, reads `chosen`, and has its own Pick button withdrawn.
_Avoid_: per-faction tree, research tree

**Research Lead**:
The Faction that contributed the most Research to the Tech that just completed. It chooses the next Tech, from version 0.07.0 out of a drawn shortlist of three rather than out of everything available; the draw always carries the Lead's own Victory gate once its prerequisites are met, so a Faction can be denied a rival's gate but never its own. The first Tech of the game is not drawn for: it is a free choice from the whole of rung 1. Decided afresh for every Tech, and the race for it stands in the top bar as one bar of the four Factions' contributions in their own colours. Since version 0.07.0 the turn cannot end while a human Lead owes the table a Tech, and Research banked while nothing is under research keeps its owner. Since version 0.07.6 a human Lead's pick is **provisional until the turn ends**: pressing a box records the choice and spends nothing, another box replaces it as often as the Lead likes, and ending the turn commits it -- at which point the Shortlist is thrown away, the banked Research pours in and the Tech may complete. A computer seat commits the instant it picks.
_Avoid_: science leader, tech leader

**Shortlist**:
The three Techs the Research Lead chooses between, drawn when a Tech completes, since version 0.07.0. The Lead's own Victory gate is always on it where its prerequisites are met; the rest come from the game's own generator. An empty shortlist is a free choice of everything available, which is how the game opens. Since version 0.07.6 it is kept for as long as a human Lead's pick can still be changed and thrown away only when the pick is committed, since redrawing it on every change would be a free reroll.
_Avoid_: options, candidates, draft, offer

**Faction window**:
The window, since version 0.08.1, that says everything about one Faction: opened by `Factions (F)` and picked from a dropdown in its own top right, defaulting to the player's own. It wears the Faction's symbol at its head and holds the live figures -- the Victory history chart (the progress lines and bars above it were cut in version 0.08.7 as a copy of the Victory window's), the income of its last turn, Blame, its **Relations** as two rows, and what it holds -- above a collapsing section carrying the Faction card itself, which the setup screen shows before a game and which had been unreachable once one began. A rival's income is shown as totals only; the building-by-building breakdown is the player's alone, and a spectator sees every seat in full.
_Avoid_: faction panel, player info, empire screen, diplomacy screen

**Top bar**:
The strip across the head of the game screen: a row of the world's and the player's figures -- Materials, Fuel, Energy, Ducats, Influence, Research and the race bar, the turn and its date, the temperature, the net ppm and the two populations -- and under it the row of buttons that open the Tech Tree, the Climate Panel, Victory, Factions and Trading, save the game and swap the map. Its height is not fixed: a Pick a Tech button or a save notice makes it taller. Since version 0.08.6 every window that opens under it reads its measured height rather than a guess of its own, so none opens over the figures.
_Avoid_: HUD, header, status bar, ribbon

**Command Cluster**:
The strip along the foot of the side panel, since version 0.07.1, holding the controls a player reaches for every turn: the Influence still unspent, Spend on whatever place is selected, Max (Defence until version 0.07.3), and End Turn. It does not scroll: the card or the Roster scrolls above it. It is the only place End Turn stands. Since version 0.08.6 the spend is set on the same slider the Smear and the Greenwash use, Max moves that slider to the bound as well as placing the order, the whole strip is a tenth larger again, and End Turn is the sun. Since version 0.08.7 the strip is two columns: the four rows (the Allotment, the slider, Spend, then Max and the every-turn tick) on the left, and the sun alone in its own column at the right, at the bottom, level with the Max row.
_Avoid_: action bar, toolbar, HUD, control panel

**Max**:
The button in the Command Cluster, since version 0.07.3, that spends everything left of the turn's Allotment on the selected place in one press, as one order to be read and cancelled like any other, and since version 0.08.6 moves the spend slider beside it to the bound as well; greyed out while nothing is selected. **Every turn** makes it a standing order on the place selected when it was ticked: at the start of each turn the whole Allotment is placed on that place as an ordinary order, never spent unwatched, until the box is unticked, the placed order is cancelled, or the place is no longer the player's.
_Avoid_: all-in, spend all, auto-spend, Defence (the retired button)

**Defence**:
Retired in version 0.07.3; see Max. It was the Command Cluster button, from version 0.07.1, that spread the turn's unspent Influence across the places a rival could take, most threatened first and each funded to safe or not at all, and the rule the computer players defended by; both went, and the computer players hold their places by their own arithmetic again.
_Avoid_: using it for anything Max now does; auto-spend, garrison, fortify, budget split

**Hab View**:
Retired in version 0.07.5; see Module. It was the window, from version 0.07.3, that showed a Space Station's or Colony's Modules as a grid of tiles. The tiles moved onto the place's own card, laid out in the same grid a Region's build slots use, and the window and its name went with the move: a Colony's tiles are simply its Modules, as a Region's boxes are simply its build slots.
_Avoid_: using it for the tiles on a Colony's card, which have no name of their own; module window, colony screen, base view, habitat panel

**Figure**:
One of the eight things the board counts and draws a glyph for: Materials, Fuel, Energy, Research, Ducats, population, Influence and Emissions. A figure's glyph carries one fill wherever it is drawn, decided by which figure it is and never by the colour of the text around it, and a word is traded for its glyph only where it names a figure -- which is to say only directly after a number, or, since version 0.07.3, where it heads a multiplier (`[research] x1.25` on a Faction card, `leans [materials]` on the start globe), with the phrase the glyph replaced on its hover.
_Avoid_: stat, counter, metric, indicator

**Roster**:
The list in the side panel of everything a Faction holds, since version 0.03 and organised in version 0.07.1: its Ships, Armies, Colonies and stations, and Regions, each group in a stable order. Since version 0.08.2 every Ship has a row of its own, by name, rather than one row for a whole stack at a Body. Since version 0.07.2 every row wears its Kind Glyph in front of its name and, where the row can want an order, a ring at its end: open while an order is still wanted this turn, filled once one is given. An Army carries no ring, since an Army keeps the stance it was last given until it is moved. (Version 0.07.1's count on the heading, clickable to filter the group, lasted one version.)
_Avoid_: unit list, overview, empire panel

**Kind Glyph**:
The small off-white mark, since version 0.07.2, drawn in front of a thing's name where the interface names it — a Roster row, a map label, a Report line — saying what kind of thing it is. Since version 0.08.2 it is NOT on a card's title: a card already says what kind of thing it is by its name and its contents, so a heading carries the Faction's symbol alone and a neutral place carries no mark at all: a warship, a Colony Ship (a Carrier wears the same), a station, a Colony, a Region, or an Army, whose glyph is the shield the Earth Map draws for it. Always off-white and never a colour of its own: on the board a colour says whose a thing is, and the glyph says what it is. A Figure's glyph, by contrast, says how much of something.
_Avoid_: unit icon, type icon, category marker

### Resources

**Materials**:
Raw metal and ore, spent to build ships, habitats and mines. Since version 0.09.0 it is the ore half of a build and Widgets are the work half: Materials are paid in full at the order, and only a Mine makes them, on Earth as a Facility and off it as a Module.
_Avoid_: minerals, supplies, ore

**Widgets**:
Work, since version 0.09.0: the second cost of everything built. A Widget is one unit of it. Widgets are a rate, not a stock: a place makes so many a turn and applies them that same turn to what is under way there, in the order the builds were given, and whatever is not applied is lost. They are never carried, traded or banked and never enter the Stockpile. A Region makes a flat four Widgets a turn and one more per Industry Level, and four more for every working Factory; a Colony or Space Station makes four from its Core Module and four for every working Factory Module. Every build carries a Widget figure, four for each turn it used to take, and completes at the Resolution its Widgets reach it. The Faction discounts on Materials reach the Widget figure too. Drawn as a cog.
_Avoid_: production, production points, shields, hammers, work units, industry

**Fuel**:
What is burned to move between Bodies. Since version 0.06.0 a transit spends it from the Ship's own Tank, which is filled at the Shipyard and refilled only by a Refuel order at a Space Station of the Ship's Faction or, since version 0.08.8, a partner's under a Refuel Accord -- since version 0.09.0 with the Ship sitting in that station's own orbit, where before any orbit at the Body would do; the Stockpile holds what Refineries make and the Trading window sells, and only a Refuel or a build moves it into a Tank. A lift from Earth spends none.
_Avoid_: propellant, rocket fuel

**Energy**:
What runs a mine, habitat or industry where it stands, drained every turn it operates.
_Avoid_: power, electricity

**Stockpile**:
The single shared pool holding all Materials, Fuel, Energy and, since version 0.03, Ducats; never Widgets, which are made and spent where they stand. It is not divided by Body, so ore mined on Mars is immediately spendable on Earth. Since version 0.06.0 its Fuel reaches a Ship only through a Refuel at a Space Station of the Ship's Faction, or at the build.
_Avoid_: central bank, per-world stocks, inventory

**Ducats**:
Money, the fourth resource since version 0.03. A controlled Region pays them from its GDP figure times its Industry Level; a Bank on Earth and a Trade Post in a Colony make more. They buy Influence at two for one, added to this turn's Allotment, and pay for Relief, Resettle, a Leapfrog, a tribute and repair points in place of Materials. What they buy in the Trading window moves in price since version 0.08.2, and every good there costs one more than it did.
_Avoid_: Ducketts, credits, money, gold, cash

**Bank**:
The Facility that makes Ducats in a Region, in proportion to the state's GDP. Since version 0.08.0 the Prospectors build the Investment Bank in its place, which makes the same Ducats and pays interest besides.
_Avoid_: treasury, mint, exchange

**Trade Post**:
The Module that makes Ducats off Earth. Since version 0.06.0 trade is a network: a Trade Post pays for every Colonist of its Faction at its Body and for every other Body where the Faction holds a Colony, a Space Station or, on Earth, a Region, so it pays for the shape of an empire rather than its size. One per Faction per Body, on the ground or on a station.
_Avoid_: market, exchange, shop

**Embassy**:
The Facility that raises Influence on Earth: while it stands and is online it adds to its controller's Allotment and raises its state's Standing for its controller each turn. Any number may stand in one state. Since version 0.09.0 it is also an **eye**: one working Embassy in a Region its holder directs lets them read every rival's income at every place on Earth, building by building, on the rival place's own card -- what the Faction window withholds, a rival's page there giving totals alone. One is enough for the whole of Earth; mothballed or offline it reads nothing.
_Avoid_: consulate, ministry, propaganda office

**Relay**:
The Module that raises Influence off Earth: it adds to its holder's Allotment and raises its Colony's Standing for its holder each turn. Since version 0.09.0 it is also an **eye**, the Embassy's counterpart off Earth: one working Relay at a Colony or station its holder directs lets them read every rival's income at every place at that Body, building by building, on the rival place's own card. One is enough for the whole Body, a Unique that does a Relay's job (the Arkwrights' Chorus) counts, and mothballed or offline it reads nothing.
_Avoid_: antenna, transmitter, beacon

### Pieces

**Observatory**:
The Module that makes Research away from Earth, since version 0.06.0: on a Colony, a Space Station or in Antarctica. It took no Body yield until version 0.07.3; since then it is multiplied by its slot's Research yield on the ground and by its Body's on a station, the first Body yield a station has read. Every Colonist living at its Colony adds one per cent to what it makes, and Public Science lifts it as it lifts a Research Lab. Its Research counts toward the Research Lead as a Lab's does.
_Avoid_: lab, laboratory, research station, institute, science module

**Solar Array**:
The Module only a Space Station holds, since version 0.06.0. It makes Energy that follows the sunlight where the station is: six a turn at Earth's distance, less than half that over Mars, nearly twice it over Venus, by the inverse square of the Body's mean distance from the Sun. Efficient Grids lifts it as it lifts a Generator, a Solar Storm turn silences it, and any number may stand on one station.
_Avoid_: solar panel, power satellite, collector

**Mass Driver**:
The Module only a ground Colony on a small world (the Moon, Phobos, Deimos) holds, since version 0.06.0, one to a Colony, behind Efficient Transit. While it works, every transit its owner's Ships fly from that Body spends four Fuel less, after every multiplier and never below one, and each Mine at its Colony makes one Materials more.
_Avoid_: catapult, launcher, railgun, launch loop

**Core Module**:
The Module, since version 0.07.5, that every Colony and every Space Station is founded with, and the whole of what a founding gives. It holds four Colonists -- a flat four, which neither Expanded Habitats nor the Arkwrights' capacity multiplier reaches -- draws a little Energy, makes four Widgets a turn since version 0.09.0 so a bare place builds a Module a turn as it did, and is never ordered, never mothballed, never decommissioned and never shut for want of Energy: it is the walls of the place rather than a building in it. It stands outside the Module count as the Archive does, and it replaced the free Habitat a ground Colony used to be founded with. Since version 0.09.1 the Core of a Colony that was **First to a Body** pays its founder 1 Influence a turn for as long as the founder directs that Colony: it sleeps under a rival who takes the place, wakes if the founder takes it back, never moves to a second Colony on the same Body, and keeps paying while the place is starved, where a Relay goes quiet. Because it holds people from the day a place stands, a station can be settled the turn it is built, which ended the deadlock whereby a bare station needed a Habitat to hold anybody and Colonists to earn the slot to build one.
_Avoid_: hub, base, the core, starter module, command module

**Module**:
A building placed inside a Colony. **Twenty kinds** since version 0.09.0. Fourteen are common to everybody: Mine, Generator, Refinery, Habitat, Shipyard, Barracks, Trade Post, Relay, the Observatory, the Institute, the Solar Array (a Space Station's alone), the Mass Driver (a small world's alone), since version 0.08.8 the Battery, the one Module that fights, and since version 0.09.0 the Factory, which makes Widgets off Earth as the Facility of that name does on it. The Core Module, which every founding gives, stands outside the count of what may be built. The Archive only the Archivists raise, one to a Faction, from its own button. The last four are the **Unique Modules**, one per Faction. (The figure read 'thirteen' from version 0.07.5 until version 0.08.3 and was wrong from the moment the Institute and the Academy were added in 0.08.0.) A Habitat holds **four** Colonists since version 0.08.1 and eight with Expanded Habitats, which raises it by four; it held eight from version 0.06.0, and since version 0.07.3 it holds the same everywhere, no world's slot making it hold more or fewer. The Core Module's flat four is reached by neither the Tech nor the Arkwrights' multiplier. Since version 0.07.5 a Colony's Modules are drawn on its own card as a grid of tiles in the same shape a Region's build slots use -- one tile per Module with its picture, dimmed while mothballed and hatched while building, a dashed tile for every free place under the cap, and the Archive on a row of its own outside the count. Since version 0.08.6 a Module ordered this turn shows in its tile at once, hatched and dimmed with *ordered* and the turns to complete on its face, and a right-click cancels the order, exactly as a Region's build slots do. Clicking a tile puts that Module's figures and its Mothball, Restart and Decommission buttons in the strip beneath; clicking a free tile puts the build buttons there, and that is the only place a Module is ordered.
_Avoid_: building, structure, facility, improvement

**Ship**:
A persistent piece that travels between Bodies. It is one of five types since version 0.09.1: Colony Ship, Carrier, Frigate, Battleship or Missile Carrier; there were four from version 0.04. It is built only at a Shipyard, on a Space Station or a Colony, and since version 0.09.1 a type may wait on a Tech, the Missile Carrier being the only one that does. It is not consumed on arrival; damage it takes persists until repaired.
_Avoid_: vessel, rocket, fleet, expedition
**Motto**:
One line a Faction says of itself, under its name on the selection card and in the in-game rulebook, since version 0.08.4 -- the Custodians' *Leave it better than we found it*, the Prospectors' *Everything has a price. We find it*, the Arkwrights' *Nothing left behind but the Earth*, the Archivists' *Everyone remembered*. It is the Faction speaking, where the blurb beneath it is the rules describing the Faction; it appears nowhere else.
_Avoid_: slogan, tagline, catchphrase

**Ship name**:
The name a Ship is given when it is built, since version 0.08.1, drawn from one of two lists -- the ships of exploration for a Colony Ship, the ships of the line for a Frigate, a Battleship, the Carrier and, since version 0.09.1, the Missile Carrier. It is unique across the whole board and is taken as the first unused name in list order, which draws no randomness and so cannot shift a seeded game. The **prefix** in front of it belongs to whoever flies the ship and is its Faction's -- TSV, PMV, ARK, ACV -- where the name belongs to the hull. The Ship's id survives on every hover, since a save file, a log line and the Report all speak in ids.
_Avoid_: callsign, registry, hull number, designation

**Colony Ship**:
The Ship type that carries Colonists, and nothing else since version 0.04. It cannot attack and is weak if caught. Since version 0.06.0 a warming Earth crowds it: lifting at Earth it may take Colonists beyond its capacity, one for every fifth of a degree the Temperature stands above +1.8, at most four, and each of those crowded aboard may die when it arrives.
_Avoid_: transport, colony (that is the settlement), settler ship

**Crowding**:
The extra Colonists a warming Earth puts aboard a Colony Ship lifting at Earth, beyond its capacity, since version 0.06.0, and the risk they run: rolled once at arrival, each crowded Colonist dies with a chance that grows with the size of the crowd. The player chooses whether to take them; the sea to Antarctica carries no crowd.
_Avoid_: overloading, coach class (that is the Arkwrights' rule), refugees aboard

**Carrier**:
The Ship type that carries one Army and nothing else, since version 0.04. Unarmed, it needs an escort and is a target for Intercept like a Colony Ship. Every landing needs one. Since version 0.08.6 an Army landed at a Colony its Faction does not direct lands on Attack and fights the turn it lands, after its own orbit has been fought -- and since version 0.09.0 the Carrier must be in the orbit that touches the place, low orbit for the ground and the station's own for a station -- and a landing that meets nobody occupies the Colony the same turn; landed at its own, it lands on Hold. An Army lands only at a Colony, never in a Region. Since version 0.08.8 every computer seat wants one where it has cause against the holder of a rival Colony off Earth, not the Prospectors alone; measured, none was built in eighty games, the appetite's terms never coinciding.
_Avoid_: troopship, transport, landing ship

**Frigate**:
The light warship type: cheap, with high Pursuit. It intercepts arriving Ships and runs down units that disengage.
_Avoid_: escort, corvette, destroyer

**Battleship**:
The heavy warship type: the most strength and hit points, low Pursuit, dear and slow to build. Since version 0.04 it carries no Army.
_Avoid_: capital ship, dreadnought, cruiser

**Missile Carrier**:
The fifth Ship type, since version 0.09.1: the hull that carries one Warhead and fires it with a Launch. Dearer than a Battleship and frailer than a Colony Ship -- no strength, no Pursuit, three hit points -- it cannot fight at all, and it waits on Missile Technology, the first Tech that gates a Ship. It is a Ship but never a warship: it holds no Orbital Control, it blockades nothing, it intercepts nobody, and under the escort rule it is struck only once its party has no warship left standing. That frailty is the counter to it by decision, and the game has no anti-missile rule.
_Avoid_: missile boat, nuke ship, ICBM, bomber

**Warhead**:
The one shot a Missile Carrier carries, since version 0.09.1, and the first thing in the game that is spent by using it. It comes with the hull at the build and is gone the moment it is fired, whether or not anything burned; a Rearm loads another, at a Colony or station of the hull's own Faction with a working Shipyard, in that place's own orbit, for Materials and Widgets -- so a carrier that has fired is out of the war for the round trip home.
_Avoid_: missile, payload, ammunition, bomb, reload

**Army**:
A ground fighting unit, of one system since version 0.08.6. A Region's Armies belong to the state and are directed by the Faction that controls it, following the state if control changes -- its **Standing Army**, and the Armies **raised** there; all march (since version 0.08.8; from 0.08.6 the Standing Army stayed at home). A raised Army's strength and hit points are its home's Industry Level plus one when it is raised, fixed for its life. A Colony's Army belongs to the Colony, exists only where it has a Barracks, only defends, and is worth the rounded average Industry Level of the Regions its raising Faction held, plus one, fixed at the raise. Any Army fights at its strength plus what it defends with: a Region's own Army its people (a working Constabulary, and calm), and any Army dug in two more. Since version 0.08.4 every Army has a name given as it is raised, from its home rather than from any Faction: an ordinal and the Region's demonym -- *the 1st Chinese Army*, *the 2nd* -- or for a Colony's, its Garrison -- *the Tycho Garrison*, *the 2nd Tycho Garrison*. The Standing Army is the first raised and so the 1st; a re-raised one takes the next number; the name survives every change of hands, since the Army is its Region's.
Since version 0.08.7 an Army's orders -- the stance row, and under a raised Army's own row its march buttons and repairs -- are given in the Armies block of its place's card, a tenth larger than the card's other text, not in a block at the card's foot.
Since version 0.09.0 **a raised Army is raised from people** (the designer: *"armies from people too"*): in a Region it takes one unit of the Region's population, one million people, at the order, on top of its Materials and Widgets and whatever its strength (`population_each` in `units.toml`), and is refused where the Region has not got it; at a Colony it takes one Colonist, and their Module slot with them, refused where fewer than two live there so the Core Module is never emptied. The Standing Army takes nobody. The people are gone: a destroyed Army returns nobody, a marching one carries nobody home, and there is no disbanding.
_Avoid_: troops, soldiers, garrison, marines

**Stack**:
Every unit of one Faction at one place: its Armies in a Region or at a Colony, its Ships at a Body. Since version 0.08.8 a stack is a thing orders are given to as well as a thing the map draws: one row of buttons marches every Army of the stack that may march, one row transits every Ship whose tank pays the leg, Repair all repairs every damaged one, and a right-click on the map after a click on the stack's shield places the whole stack's march. The orders are still the units' own, placed together, so the arrival and the Battle are each unit's; nothing is a stack in the rules. The computer marches a stack where one Army alone would not clear its bar.
_Avoid_: fleet, army group, formation, squadron

**Escort**:
A warship standing engaged in a Battle beside an unarmed hull of its Faction, a Colony Ship or a Carrier. Since version 0.08.8 the escort takes the fire: while a party has a warship engaged, every hit on that party lands on one of its warships, and an unarmed hull is neither struck nor pursued until no warship of its remains engaged. A Battery counts as armed for this, and every Army is. The word names a role in the Battle, not an order: any warship in the line is the escort of the unarmed hulls beside it.
_Avoid_: guard, convoy, screen

**Standing Army**:
The Armies a Region keeps on its own, sized by its card and replenished a little each turn, whether or not any Faction controls it. Named like any other Army since version 0.08.4 -- the one the game begins with is *the 1st Chinese Army*. Its strength and hit points are the Region's Industry Level plus one plus the steps it has **armed** by: since version 0.08.5 one for every attack a neutral Region has held against, and since version 0.08.6 two at the Income a threat appears next door (a foreign Army in a neighbouring Region, or a neighbour under Occupation), once per threat, where a Levy used to be raised; there is no ceiling, so a long-neutral Region is a fortress by force and Influence is the cheap way in. Since version 0.08.6 **a Region's defence is its people**, as defence: while it defends it fights one stronger for a working Constabulary and one stronger while Unrest is under its threshold, both read live and neither adding hit points; and an Army whose damage reaches its strength is destroyed rather than standing at nought. Its holder may march it (since version 0.08.8; from 0.08.6 it stayed at home): at home it defends with its people, the Constabulary and the calm; away it is an Army like any other, and a threat next door like any other. Destroyed, it returns at strength one two Incomes later rather than the next (`respawn_incomes` in data since version 0.08.8), so a won Battle opens a window.
_Avoid_: garrison, militia, defence value

**Levy**:
Retired in version 0.08.6. From version 0.08.5 it was the second Army a neutral Region raised while a foreign Army stood next door or a neighbour was Occupied, at Industry Level plus two, standing down when the threat passed. Now the same threat **arms the Standing Army for good**, two steps at the Income the threat appears, once per threat, with no ceiling. A Region still arms whatever the neighbouring Army's stance, so nobody's orders are disclosed by it.
_Avoid_: militia, reserves, conscripts, second Standing Army

**Barracks**:
The Module that lets a Colony hold and build a defensive Army. A Colony without one has no defenders. Since version 0.08.6 the Army it raises is worth the rounded average Industry Level of the Regions its Faction holds, plus one, fixed at the raise; the Barracks is not a police force, so that Army defends with Dig In alone. Since version 0.09.0 the raise takes one Colonist from the Colony (`colonists_each` in `units.toml`), refused at fewer than two, and the Colonist does not come back.
_Avoid_: fort, garrison, base

**Battery**:
The one Module that fights, since version 0.08.8, on a Colony or a Space Station, with a strength and hit points on its card (four and six) that Hardened Hulls does not reach. It stands in the line of any Battle fought in **its own orbit**, on its owner's side, as though on Hold, and never disengages, since it cannot leave; a rival stack ordered Attack in that orbit fights it whether or not its owner has a Ship there, and destroying it is the Battle's, at rung 3 like any other. Damage it takes is repaired with Materials at its own Colony, as a Ship's is at a yard; at its hit points it is destroyed and gone. While one stands and works, no rival holds Control in the orbit it covers and no Blockade shuts it: since version 0.09.0 a ground Colony's Battery covers **low orbit**, where Orbital Control is held, so a Colony that arms itself keeps a fleet from landing without standing in for one; a station's Battery covers **its station's orbit**, shielding the station rather than the whole sky, which narrows what version 0.08.8 gave it. It is not a ground defender: an Army that lands still meets the Colony's Army or nobody. Mothballed or offline it neither fires nor denies, as no Module that is not working does anything. The computer wants one where a rival warship stands at the Body or a rival Carrier is inbound, one per Colony.
_Avoid_: turret, gun, defence platform, orbital defence, fort

**Colonist**:
A person counted in the population of a Colony or Region. Since version 0.05.5 Colonists are built: they are recruited in a Region as Pioneers, are carried by Ships and held by Habitats, and are never spent as a resource. One Colonist is one unit of population -- one million people since version 0.09.0 (the designer: *"pop 1 per million"*), five million from version 0.07.3 until then -- and the top bar counts every Colonist living off Earth as the space population beside Earth's. The unit is `people_per_unit` in `climate.toml` since 0.09.0, a code constant before.
_Avoid_: settler, crew, worker, population resource

**Pioneer**:
A Colonist recruited in a Region and not yet gone: up to four a turn per Faction (eight for the Arkwrights), in one state the Faction directs, each taking one unit of the state's population -- one million people since version 0.09.0; five million from version 0.07.3 until then; a tenth of a hundred million, ten million, before that -- on the state's card at End Turn. A batch takes half a point off the state's Unrest. A working Launch Site lifts Pioneers onto a Ship, or, since version 0.07.3, straight onto a Space Station of their Faction's over Earth, as many as its Habitats have room for, aboard at that turn's Resolution -- a launch, with no crowd; and once the Antarctic ice is open the sea takes them to Antarctica in a turn with no launch. They are people of their state until they leave it. The Arkwrights alone begin the game with two already waiting (since version 0.08.6), recruited by nobody and paid for by no population.
Called an **Emigrant** until version 0.08.3, and **recruited** where it was mustered. The designer's reason was the word rather than the rule: *"it's more about the word being politically loaded and racked with connotation"*. The rule is untouched -- a Pioneer is the same stage of a Colonist's life the Emigrant was, which is why the distinction was kept rather than folded into Colonist: **Coach Class** and the **Spaceport** are both stated in terms of it, and "every Colonist costs their Region twice the population" would have been false.
The engine still spells the field, the order and the Report keys `emigrant`: they are not read by a player, and renaming the two serialized fields on every Nation State would have broken every save for no visible gain.
_Avoid_: emigrant, settler, migrant, passenger, colonist-in-waiting, pilgrim

**Region**:
One of fourteen territories of the Earth Map that a Faction can control and build in, called a Region until version 0.07.2 and since then **named for its Nation** -- the power that leads it -- so the territory that was South Asia is India and the one that was Europe is the European Union. Eight until version 0.05, which split Africa at the Sahara, Asia into East, South and South-East, and Central America and the Caribbean out of North America; Antarctica left the list in version 0.04 to become Earth's Colony Slots. Each carries a population -- the whole territory's, not its Nation's alone, counted in units of one million people since version 0.09.0 (five million from 0.07.3 until then, hundreds of millions before that) and written on its card as `Region population 1454.5 (1.45B)`, the unit figure to one decimal with the people in brackets -- an Industry Level, a Resource Lean, a Baseline Emissions figure, an Education Level, a GDP, an Influence value, an Unrest figure and a Standing Army. Armies move only between neighbouring Regions.
_Avoid_: nation state, country, territory, state, continent, bloc

**Nation**:
The power a Region is named for and whose flag its card wears, since version 0.07.2: China, India, the United States, Brazil, Russia, Australia, the European Union, Iran, Egypt, Nigeria, Indonesia, Mexico and -- since ticket #125 cut them out of East Asia and the Middle East in that same version -- Japan and Saudi Arabia. A Nation is a name and a flag; the Region is the thing on the board that is held, built in and fought over, and it is larger than its Nation -- India the Region holds Pakistan, Bangladesh and Sri Lanka. The choice of Nation for a Region is the designer's and is a statement about the world of 2030.
_Avoid_: primary power (the charting phrase; not the game's word), country, capital, leader

### Earth

**Facility**:
A building placed in a Region. Sixteen kinds since version 0.09.0: Factory, Mine, Power Plant, Refinery, Launch Site, Research Lab, Bank, Embassy, Constabulary, the School, the four Unique Facilities, and the Sea Wall and the Scrubber, which take no build slot (the Sea Wall stood in a Coastal Slot until version 0.05.5). Since version 0.09.0 the Factory makes Widgets and the Mine, new that version, makes the Materials the Factory used to; the two carry the names of the Modules that do the same jobs off Earth, as the Refinery always has. Since version 0.04 a Launch Site builds no Ship: it lifts Pioneers and Armies from its state into orbit, and each lift is a launch.
_Avoid_: item, building, module (that is the Colony word), structure

**Cheap Industry**:
The first clause of the Prospectors' signature rule: raising a Region's Industry Level costs them half, and since version 0.05.5 every Facility and Colony Module costs them 15% less. The second is the Strip Permit.
_Avoid_: industry discount, cheap building

**Restoration**:
Retired in version 0.05; see Scrubber. It was the Custodians' signature rule: Energy spent in a turn enlarged the Natural Sink for that turn only.
_Avoid_: using it for anything the Scrubber now does

**Unique Facility**:
A Facility one Faction builds in place of a common one, at the common price -- the same Materials, the same build turns, the same upkeep, the same output, the same slot -- with its own name, its own icon and its own row on that Faction's build list, and one clause the common building does not have. One per Faction as a rule of the system, from version 0.08.0: the Prospectors' Investment Bank, the Arkwrights' Spaceport, the Archivists' Reactor and the Custodians' Academy. A Faction never builds the common version of a job it has a Unique Facility for, and never builds another Faction's.
When its place changes hands it is **rolled like any other building** (since version 0.08.6, and in truth since the roll was written: the glossary said otherwise), and one that survives keeps standing and pays its new holder the same clause it paid its builder, which is the opposite of the Scrubber and the Archive, which are destroyed outright. The mirror does not hold -- a common building already standing does not convert when the Faction whose Unique version it is takes the place. Only a CONTROLLER collects: an occupier pays the upkeep and draws nothing.
_Avoid_: unique building (the glossary keeps Facility and Module apart), signature building, faction building, wonder

**Unique Module**:
The same idea off Earth: a Module one Faction builds in place of a common one, at the common price. Since version 0.08.3 there are **four, one for every Faction**, so each Faction now has a Unique Facility on Earth and a Unique Module off it. The Custodians' **Academy** replaces the Institute and wears the same name their Unique Facility does on Earth; the Archivists' **Heliostat** replaces the Solar Array; the Prospectors' **Exchange** replaces the Trade Post; the Arkwrights' **Chorus** replaces the Relay. Version 0.08.0 had only the Academy.
A Unique does its common sibling's **job**, which is what makes the rest of the game reach it: a Tech that lifts a Relay lifts a Chorus, a Discovery on a Solar Array lands on a Heliostat, and a cap counting Trade Posts counts Exchanges. The Archive is NOT one: it is a Faction-only Module that is destroyed on capture, which is the opposite rule.
_Avoid_: unique building, signature module

**Heliostat**:
The Archivists' Unique Module since version 0.08.3, replacing the Solar Array at the common price: a Space Station's alone, 25 Materials, two turns, no Energy upkeep, and the same 6 Energy scaled by the inverse square of its Body's distance from the Sun. Its clause is **one more Energy, added after that scaling**, so the extra point is worth the same at Mars as at Venus rather than 0.43 of a point at one and 1.91 at the other. A Solar Storm silences it as it silences a Solar Array.
_Avoid_: mirror, solar farm, collector, panel

**Exchange**:
The Prospectors' Unique Module since version 0.08.3, replacing the Trade Post at the common price and paying by the same network rule. Its clause is **one more Ducat a turn, flat, added after the output multiplier** — flat for the Academy's reason: one run through the largest output multiplier in the game floors back to one, so a captured Exchange pays its captor exactly what it paid its builder. One per Faction per Body still applies.
_Avoid_: market, bourse, trading house, the Trading window (which is a different thing entirely)

**Chorus**:
The Arkwrights' Unique Module since version 0.08.3, replacing the Relay at the common price and carrying everything a Relay carries, Relay Networks included. Its clause is **one more Influence in its holder's Allotment for every six Colonists at its own Colony, rounded down** — its own Colony, as an Observatory reads its own Colony's people, not its whole Body. The more people stand there, the further the voice carries, which is the Faction whose entire game is moving people. It adds nothing to **Standing**: "+1 Influence" has meant the Allotment, the Faction's diplomatic budget everywhere, since version 0.08.3 settled it on the Relay.
_Avoid_: choir, broadcast, transmitter, crowd

**Scrubber**:
The Custodians' signature Facility, which only they build and only in a Region they control. It takes no build slot, draws Energy, emits nothing, and while it is online it enlarges the Natural Sink and lowers its state's Unrest every turn. How many one state may hold follows its population, and they are destroyed outright if the state changes hands.
_Avoid_: carbon capture, cleanup, restoration (the retired rule), terraforming, filter

**Leapfrog**:
The Custodians' other signature clause: an Orders-phase order paid in Ducats on a Region they control, which lowers that state's per-person Emissions coefficient by one Industry Level's worth, for good. It may be bought any number of times, and never takes the figure below the base every state pays. Since version 0.08.3 it needs **Three Turns Held**.
_Avoid_: clean development, technology transfer, offset, upgrade

**Strip Permit**:
The Prospectors' other signature clause: an Orders-phase order, free, that may be taken once in a Region's whole life. For three turns every Facility there produces double; when it ends, that state's Baseline Emissions and its Unrest rise for good. Since version 0.08.3 it needs **Three Turns Held**.
_Avoid_: licence, boom, overdrive, exploitation

**Three Turns Held**:
The condition the three orders that remake a Region for good -- the **Leapfrog**, the **Strip Permit** and the **Exodus Call** -- have carried since version 0.08.3: the Faction must have held that Region for three whole turns. The turn of the taking does not count, so a Region taken on turn 10 opens its Faction order on turn 13. Holding is what the clock measures, not holding continuously by that Faction alone: it restarts whenever the **controller changes**, and a Region written back to the same Faction keeps the clock it had.
It is a **delay, not a prohibition**. Measured before it was adopted, the Prospectors held nine of their nine Regions for three turns or more, and issued their permits at an age of nought or one turn; the rule moves those permits later rather than abolishing them.
_Avoid_: cooldown, lockout, occupation, tenure, grace period

**Mothball**:
The Orders-phase order that stands a Facility or a Module down. A mothballed building produces nothing, pays no Energy upkeep, emits nothing, is online for no rule, and keeps its slot. It is free, it takes effect at the Resolution, it raises the Unrest of a Region it happens in, and only a Restart brings the building back, for Materials and a turn.
_Avoid_: pause, disable, switch off, idle, shut down (that is the Energy shortfall rule)

**Offline**:
A standing building that is not mothballed and is not running. Four things put one offline: the Energy shortfall rule at Income (buildings shut one at a time, dearest upkeep first, Module before Facility, the Core never, until the bill is met); an Event card (Wildfire, Labour Dispute, Launch Pad Fire, Reactor Leak) until the next Resolution; a grid failure, which shuts every Module in the Colony; and Occupation, which shuts the Colony's Archive whoever directs it. An offline building makes nothing, keeps its slot, and pays no upkeep. Two things look like it and are not: a blockaded station's Modules make nothing and still pay, and a Facility in a Region nobody directs is idle. Since version 0.08.7 an offline building's box is dimmed with the word *offline* in its corner, and its hover names the cause.
_Avoid_: broken, damaged, disabled, unpowered, dark

**Production Moved**:
The Custodians' third signature rule, since version 0.06.0. While a Factory, Mine, Power Plant, Refinery or Research Lab of theirs on Earth is mothballed, one Factory, Mine, Generator, Refinery or Observatory of theirs off Earth makes double, one Facility for one Module of the same job, the most productive undoubled Module first, on its final figure. Since version 0.09.0 the Factory pairs with the Factory Module and the Mine with the Mine, so a mothballed Earth Factory doubles Widgets off Earth, which is what the name always said; before 0.09.0 the Factory paired with the Mine. A Restart ends it; with no idle Facility of the pair there is no bonus.
_Avoid_: offshoring, relocation bonus, the mothball bonus

**Decommission**:
The Orders-phase order that takes a Facility or a Module down for good: a turn later half its Materials come back, its slot is free, and it is gone. In a Region it raises Unrest more than a Mothball does; in a Colony it raises none.
_Avoid_: demolish, scrap, sell, destroy

**Industry Level**:
How built-up a Region is. Together with the state's size it sets how many Facilities fit, and every slot it adds is an Inland Slot. It scales the state's emissions. Since version 0.09.0 it adds one Widget a turn per level to the flat four every Region makes with no Factory, so a built-up Region builds a little faster on its own. Raising it is a build action.
_Avoid_: development, tier, infrastructure

**Neutral Development**:
What a Region nobody holds does for itself: every ninth turn of unbroken neutrality it raises its own Industry Level by one and wakes one of its own Facilities to run itself, emitting to nobody and making nothing for anyone, until it is as built-up as a neutral state gets. A world already too hot, or a population already too restive, stops it. The clock is the state's own, starts again whenever it is freed, and the states neutral at the start are staggered so they do not all step together.
_Avoid_: growth, expansion, auto-build, industrialization, AI development

**Education Level**:
How well a place is schooled. It began as a fixed figure on a Region's card, taken from real-world values, that multiplies the Research each Lab there produces. Since version 0.08.0 it MOVES and it is no longer a Region's alone: a School raises it a step a turn to a ceiling and it falls back at the same rate when the School stops; a Colony or a Space Station has one too, the weighted average of the schooling the settlers brought from the Regions they were mustered in, which an Institute raises the same way. It does three jobs now -- it multiplies a Lab's and an Observatory's Research, it moderates what a large population adds to Research, and it is what Resistance reads.
_Avoid_: literacy, science level; schooling is the ACCUMULATED part alone, not the whole figure

**Resource Lean**:
The one of Materials, Fuel or Energy a Region is naturally good at producing.
_Avoid_: specialty, bonus, affinity

**Baseline Emissions**:
How dirty a Region's industry is before any Facility is built there.
_Avoid_: pollution rating, carbon score

**Build Slot**:
One of the places a Facility occupies in a Region. A state has Size + Industry Level + three of them, and each is either a **Coastal Slot** or an **Inland Slot**: the sea takes only the first, and every slot a raise of the Industry Level adds is one of the second. Its start Facilities stand on the coast first; a later build fills an inland slot while one is free. Since version 0.07.3 the card draws them as boxes -- a picture on a dark box, dimmed while mothballed, hatched while building, dashed while free, a coastal box outlined in blue and an inland one in grey -- and a slot the sea has taken stands under water, three quarters flooded with the drowned building dimmed beneath. Since version 0.08.6 a building shows in its box **the moment it is ordered**, before End Turn, hatched and dimmed with *ordered* in one corner and the turns to complete in the other, and a right-click on it cancels the order; once the turn ends the word becomes *building* and the count runs down, and the box can no longer be clicked. Since version 0.08.2 a free box the player could actually build in says so, where one they could not keeps the bare word. Since version 0.07.4 every box says on hover what its row says -- the kind, its figures, and the upkeep, Emissions and coastal rules -- and a free, building or flooded box says what it is; the Scrubber and the Sea Wall, which take no slot, stand beneath the boxes with their build buttons, and a Facility that takes a slot is built by clicking a free box and nowhere else.
_Avoid_: building slot, plot, space, capacity

**Coastal Slot**:
A build slot on a Region's coast, where the sea can reach it. A state begins with two per point of Coastal Exposure (three before version 0.05.5), never more than its start slots less one, and since version 0.08.5 it gains one more at every Sea Level rise that reaches it, turned from an Inland Slot, wall or no wall. A Facility standing in one the sea takes is destroyed, oldest first.
_Avoid_: shore slot, waterfront, flood zone

**Inland Slot**:
A build slot the sea does not take: the start slots the coastal ones leave over, and every slot a raise of the Industry Level adds. Since version 0.08.5 the sea reaches it another way: every rise turns one Inland Slot of the state into a Coastal Slot, an empty one first and otherwise the oldest inland Facility's with the Facility on it, so the next rise can take it. A raised slot turns like any other.
_Avoid_: safe slot, interior, highland

**Coastal Exposure**:
How much of a Region stands on the coast, a figure on its card. It sets how many Coastal Slots the state begins with, two per point since version 0.05.5 (three before), and how many of them each Sea Level threshold takes. A state with no Coastal Slots left loses no more of them that rise, though the threshold still drives out its people, raises its Unrest and, since version 0.08.5, turns one Inland Slot coastal. A state at exposure 0 would be landlocked and spared all of it; none on the board is.
_Avoid_: coastline, vulnerability, flood risk

**Sea Wall**:
The Facility that holds the sea off: it takes no build slot (since version 0.05.5; a Coastal Slot before), at most one to a state, and while it is working every Sea Level threshold of any kind takes no slots from the state; since version 0.08.5 the threshold still turns one Inland Slot coastal behind the wall, which holds the taking off and not the turning. Since version 0.08.4 it **stands** through them -- from version 0.05 to 0.08.3 it absorbed one and was destroyed doing it -- and each rise it has held adds half a Material a turn to its keep, paid at Income; a seat short of Materials leaves it unkept that turn, and an unkept wall holds nothing. A Storm Surge that breaks on a standing wall no longer brings a threshold forward: the wall holds, and the Facilities in the state's Coastal Slots make three tenths less at the next Income. It needs Coastal Engineering.
_Avoid_: dyke, levee, barrier, flood defence

**Coastal Engineering**:
The Industry Tech that unlocks the Sea Wall, and does nothing else.
_Avoid_: sea defence, civil engineering, hydrology

**Unrest**:
How restive a Region's people are, a figure from 0 to 10 on its card, moving in halves. Heat, the rising sea, the Climate cards, Occupation and arriving Refugees raise it; it falls on its own every turn except the turn the state changed hands, so a rise and the fall net out, and it falls further to Relief, a Constabulary and a Scrubber. Past its first threshold the Standing Army stops replenishing, past its second the state's Facilities run at half, and at the top the state throws its controller off and goes neutral.
_Avoid_: unhappiness, morale, stability, dissent, revolt meter

**Agitate**:
Relief's mirror, since version 0.08.4: an order paid in Ducats and Influence on a Region a rival controls, raising its Unrest by one at Resolution -- once a turn per Region per Faction, so a Region cannot be bought off its holder in a turn, and dearer than Relief so a duel is not settled by income alone. A working Constabulary halves it. It is an offence, and the holder's Report names who paid. The computer seats use it in the Regions of a rival they are Cold or Hostile toward, most eagerly where Unrest already stands past the first threshold.
_Avoid_: incite, destabilise, sabotage, foment

**Relief**:
The Orders-phase order that buys a Region's calm: Ducats spent on a state you direct to lower its Unrest by one, as many times in a turn as you can pay for.
_Avoid_: aid, welfare, subsidy, bribe, appeasement

**School**:
The Facility that teaches, from version 0.08.0. At most one per Region, buildable by any Faction. While it stands and is online it raises its Region's Education Level by a step a turn to a ceiling; when it stops, the figure falls back at the same rate and stops at the card's own. It is the only thing in the game that moves an Education Level on Earth.
_Avoid_: university, college, academy (that is the Custodians' version), institute (that is the version off Earth)

**Institute**:
The School's form off Earth: a Module at a Colony or a Space Station, at most one per place, doing the same work at the same step to the same ceiling. When it stops, the figure falls back to the average the settlers brought with them.
_Avoid_: school (that is the Earth version), laboratory, observatory

**Academy**:
The Custodians' Unique Facility on Earth and their Unique Module off it -- one name in both halves. It does everything a School and an Institute do, and pays its holder a flat Ducat a turn besides while it is online, wherever it stands.
_Avoid_: school, institute, university

**Constabulary**:
The Facility that holds a Region's Unrest down: while it stands and is online it lowers Unrest every turn whatever else happened that turn, and softens what the climate and arriving Refugees add. At most one stands in a state. Since version 0.08.0 it is also a defensive building: while it stands and is online the challenge margin in that Region is 25 instead of 20, for EVERY challenger and whoever built it -- a police force serves the government of the day. Since version 0.08.6 it fights too: while it works, the Region's Standing Army defends one stronger; it adds no hit points.
_Avoid_: police, militia, garrison, barracks (that is the Colony word)

**Refugees**:
The people who leave a Region for its neighbours when the heat or the sea takes their homes, instead of simply being lost. They are added to the state that receives them, and their arrival raises its Unrest, charged on everyone who arrived rather than on the net. Since version 0.07.6 the Report says one **net migration** line per Region rather than one per flow: a Region that gained says how many it took in and what its Unrest did, one that lost names the largest cause that drove them out, and one whose flows cancel says nothing at all. The game's log keeps the whole record, one line per flow.
_Avoid_: migrants, displaced, evacuees, exodus, immigration

**Resettle**:
The Orders-phase order that steers Refugees: once a turn, for Ducats, every flow leaving the states a Faction directs goes to one Region of its choosing instead of to the neighbours, and its Standing there rises.
_Avoid_: relocate, evacuate, transfer, deport

**Blame**:
The CO2 a Faction is answerable for over the whole game, as a **ledger**: it begins from physics -- everything the sources it controlled has emitted, since version 0.08.5 the hits its Battles on Earth landed and the buildings its takings burned among them, less everything it has taken back (its Scrubbers' removal, and since version 0.08.4 what the Custodians' Research Directive has added to the Natural Sink, counted every Climate phase it stands) -- and since version 0.08.4 it is moved by deals and words as well: carbon credits bought take ppm off the buyer's ledger and put it on the seller's, a Smear campaign lays ppm on its target's, and since version 0.08.5 a Greenwash takes ppm off the campaigner's own. The share every rule reads is computed from the ledger, so it can diverge from what a Faction physically put in the air. Never less than nothing. What no Faction controls is nobody's. A Faction's share of the four Factions' Blame, when it rises above a fair quarter, makes every Region it does not hold harder for it to win over; and since version 0.08.4 it moderates how fast the Faction's Standing decays on Regions it does not hold -- 3 a turn at a share of a half or more, 1 at an eighth or less, 2 between. Since version 0.08.2 it does a second thing: the other Factions hold it against them, each by its own measure -- the Custodians minding twice as much as the Archivists, the Arkwrights half as much, the Prospectors not at all -- so a dirty Faction loses the room to strike an Accord as well as the ground it wanted.
_Avoid_: carbon debt, guilt, pollution score, emissions total, footprint

**Carbon credit**:
A ppm of Blame bought off a Faction's ledger, since version 0.08.4, from the Custodians and nobody else: they set the ppm they offer a turn, standing until changed, and any other Faction may buy up to ten ppm of it a turn at one Ducat a ppm times how the Custodians think of the buyer (Friendly half, Cordial three quarters, Wary half again, Cold double, Hostile refused). A credit bought comes off the buyer's Blame for good; it comes off the Custodians' Blame Credit, and past what they hold it goes onto their own ledger as Blame taken -- they may sell more than they have, which is their strategic choice to make. An offer is shared first come first served, a buyer left short gets its Ducats back, and a purchase is an act of friendship both ways. Since version 0.08.5 the trade is made on the Faction window and not the Trading window: the Custodians offer from their own page, and every other Faction requests from the Custodians' page, beside the Accords and the Smear; a request is filled from the standing offer, the word having changed and the rule not. The computer Custodians offer their whole credit while their own share is under the fair quarter and refuse when it is not.
_Avoid_: offset, indulgence, Blame credit (that is what the seller holds), carbon permit

**Smear**:
Influence spent on a rival Faction rather than a place, since version 0.08.4: every point lays two ppm on the rival's Blame ledger for good, moving the share every rule reads. One campaign a turn against each rival, of any amount the Allotment covers -- set since version 0.08.6 on a slider in single points, from nought to the turn's whole Influence with the part already ordered elsewhere greyed, and spent with a button; an offence, and the Report names who paid. Called a smear because it inflates Blame above the ppm the rival actually produced -- a kind of a lie -- which is why the ledger and not the physics is what the rules read. The computer seats use it against a rival they are Cold or Hostile toward whose share stands above the fair quarter.
_Avoid_: propaganda, denounce, expose, libel

**Greenwash**:
Influence spent on a Faction's own name, with a Ducat beside every point, since version 0.08.5: every point takes two ppm off the Faction's own Blame ledger for good, the whole ledger and never below nothing, moving the share every rule reads. One campaign a turn, of any amount the Allotment and the Ducats cover -- set since version 0.08.6 on the Smear's slider with the Ducats painted as a second bound where they bite first; public and no offence, so the Report says who greenwashed and a rival may answer with a Smear. The Smear's mirror, at the designer's word, and open to every Faction, the Custodians included. Called a greenwash because the Blame falls without a gram of CO2 leaving the air. The computer seats use it when their own share stands above the fair quarter and carbon credits are not to be had.
_Avoid_: propaganda, PR, spin, whitewash, clean-up, self-smear

**Blame Credit**:
The CO2 a Faction has removed over the whole game, in ppm -- what its Scrubbers took out of the air, and for the Custodians what their Research Directive adds to the Natural Sink, counted every Climate phase. Since version 0.08.4; before that the word meant removal beyond everything the Faction had ever emitted, a figure measured at zero in every game, so it was read by no rule and held by nobody. It is the supply a carbon credit is sold from, less what has been sold. The panels read *answerable for N ppm (emitted E, removed R in credit)*.
_Avoid_: offset, carbon credit (that is the thing bought and sold), removal total

**Influence**:
A Faction's claim on a Region or a Colony, spent from a per-turn Allotment onto a place, where it becomes the Faction's Standing there. A neutral place goes to the first Standing at its threshold, and when two claimants reach it on the same turn at the same Standing the lot decides between them (since version 0.05.5); a controlled place goes to a rival whose Standing is at least the controller's plus the challenge margin (20 since version 0.05.5, 10 from version 0.04, and 25 since version 0.08.0 in a Region where a Constabulary stands and is online) and at least the threshold, which since version 0.05 each Faction reads for itself, since its Blame raises it on every Region it does not hold. The margin is 20 everywhere, on Earth and off it: version 0.09.0 doubled the thresholds off Earth and deliberately left the margin alone, so a settled Colony is no dearer to take than it was and what got dearer is a place worth taking on its threshold. Since version 0.08.0 the Influence an OUTSIDER spends is divided by the place's Resistance before it becomes Standing. A holder is never tied with a challenger, and keeps the place when two challengers tie. Since version 0.09.1 a Faction is also paid Influence for being **First to a Body**. Since version 0.05.5 every Faction begins with a Standing on its start state equal to that state's threshold: a claim on its home from turn 1.
_Avoid_: diplomacy points, favour, reputation

**Standing**:
How much Influence a Faction has built up on one place. Since version 0.03 it persists: it is never wiped when the place changes hands, it decays 1 a turn on a place the Faction controls and 2 a turn elsewhere when nothing is spent -- on a Region the Faction does not hold, since version 0.08.4, 3 a turn if its Blame share stands at a half or more and 1 if at an eighth or less -- and spending on a place you hold raises it. Since version 0.08.0 it is no longer simply the Influence spent: an outsider's spending is divided by the place's Resistance first, so 30 Influence on a well-schooled Region buys 27 Standing. A controller converts in full. Being **First to a Body** buys no Standing anywhere: it pays into the Allotment, which is what Standing is bought with.
_Avoid_: accumulation, influence points, loyalty

**Threshold**:
The Standing a Faction must reach to take a place. A Region's is a base plus a figure for each step of the state's size; a place off Earth's is a base plus a figure for each of its Colonists -- since version 0.09.0 **40 plus 20 a Colonist**, where it was 10 a Colonist with 20 more for a Space Station, and the station's base of 40 REPLACES the Colony's rather than adding to it, so a station and a ground Colony of the same crew cost the same. Before 0.09.0 a Colony nobody had moved into was worth nothing at all; the base is what makes an empty place cost something. Green Consensus lowers every threshold by a quarter, and since version 0.05 each Faction reads its own, since its Blame raises it on every place it does not hold. A place already held wants the holder's Standing plus the challenge margin as well, so the threshold is the floor and never the whole price. Since version 0.08.2 that margin is a little dearer for a challenger the holder thinks badly of, so the same Region costs more to take from somebody you have crossed than from somebody you have not. Schooling does NOT move the threshold: Resistance taxes the spending instead. The word has a second, unrelated sense in the climate rules, where a Break or a Sea Level threshold is a Temperature; this entry is the Influence one.
_Avoid_: cost, price, target (for the Influence sense); the climate sense, which is a Break or a Sea Level threshold

**Challenger line**:
The line in a held place's Standings block, since version 0.08.4, naming the rival nearest to taking it by Influence -- nearest its own price, not the highest Standing -- and how far off it stands: *The Prospectors stand at 47; they take this at 70.* Amber, with *Spend here to stay ahead.*, when they press within two steps; the arithmetic on its hover; *No rival has a Standing here.* when nobody does.
_Avoid_: rival line, warning line, contest indicator

**Threat line**:
The challenger line's military counterpart on a held Region, since version 0.08.7: the rival raised Army standing in a neighbouring Region with the best first-exchange odds against the Region's defenders, named with its strength and the Region it stands in -- *The Prospectors' 2nd Russian Army, strength 3, stands next door in Russia.* -- and never with its stance, as a Region arms stance-blind. Amber, with *Dig In here to hold it.*, when their odds reach the bar the computer seats attack at, which is the computer's habit and not a rule; the arithmetic on its hover; no line at all when nobody stands next door. Regions only: a Colony has no neighbours.
_Avoid_: danger line, enemy line, alert

**Resistance**:
How hard a place is to sway, from how well it is schooled, from version 0.08.0. The Influence an OUTSIDER spends there is divided by it before it becomes Standing, rounded down; the controller converts in full. A place at the neutral pivot has no effect; below it an outsider's spending goes further, above it less, to a band either side. It touches neither the threshold nor the challenge margin -- it taxes the spending, not the gate -- which is why a well-schooled Region is harder to buy without being harder to reach.
_Avoid_: defence, loyalty, resilience, stubbornness

**Relations**:
What one Faction thinks of another, from version 0.08.0. One score per ORDERED PAIR, so twelve in a four-seat game and the Arkwrights' view of the Prospectors is a different number from the Prospectors' view of the Arkwrights. Since version 0.08.2 it is two things added together: the **deeds**, everything the pair has done to each other, and a **Blame term** read afresh every turn off the other Faction's share of the four Factions' Blame. Only the shown sum is held to the scale; the deeds figure alone may climb past it, which is what lets a pair carrying a heavy Blame penalty still reach the top on deeds. It is read as six **levels** -- Friendly, Cordial, Neutral, Wary, Cold, Hostile -- and the level is what a player reasons with, the number being the audit trail. An OFFENDING TURN costs what the acts in it are worth rather than a flat one, to a limit; quiet mends it below neutral and lets it lapse half as fast above. A pair crossed often enough carries a **floor** it can never recover above again, which only an Accord kept will lift. Since version 0.08.2 it is no longer only read: a rival that holds you below neutral defends its places against you a little harder, and an Accord wants a level it will not be struck below.
_Avoid_: diplomacy, alliance, opinion, reputation (that is close to Standing)

**Accord**:
A bargain between two Factions, from version 0.08.2, holding one or more **Terms**: non-aggression, passage, refuel, a tribute, or a research agreement. Either side may propose one and either may refuse, and a refusal is not an offence. Ending one takes a turn's notice and costs nothing -- the notice is a turn of warning to everybody watching -- where acting against a term while it still stands is an offence and ends the whole Accord at once. An Accord kept for a run of turns is one of only two acts that raise Relations, and the only thing that lifts a pair's floor. A Faction may hold Accords with two rivals at once, including with two who are fighting each other.
_Avoid_: diplomacy, treaty, alliance, pact, deal

**Terms**:
What is inside an Accord. **Non-aggression**: neither spends Influence on a place the other holds, nor opens a Battle against them -- not forbidden by the rules, but paid for and fatal to the Accord. **Passage**: since version 0.08.8 a rule for Armies as well as Ships: either's Armies may march into the other's held Regions without attacking, arriving on Hold, fighting nobody and defending nothing while the Accord stands, and on Attack at the next Resolution once it ends, as any rival Army would be; neither intercepts the other's Ships, and a Blockade does not shut the other out of the slot. Without it a march into a held Region is an attack, charged as one when the Battle is fought. **Refuel**: either may Refuel at the other's Space Stations, from their own Stockpile, since version 0.08.8 in the rules and not only in the text; it costs the granting side nothing, so the computer accepts it at Wary or better and offers it, in its non-aggression offer, where the other holds a station at a Body it has Ships or a Colony at and no station of its own. **Tribute**: a fixed gift, one a turn to a Faction, which raises their view of you by one; it is a single turn's act rather than a standing term. **A research agreement**: both parties' Research rises a tenth while it stands, and it wants Friendly on both sides to strike -- checked at that moment and never again.
_Avoid_: clause, article, condition (that is close to Victory Condition), provision

**Allotment**:
The amount of Influence a Faction receives each turn, split freely across any number of targets during the Orders phase. It is a base plus the Influence value of every Region the Faction controls (since version 0.03 each state carries its own value, from its economic and military weight), and it does not carry over. Two things are added to it at face value, after the Faction multiplier rather than inside it: what the Faction's Spaceports earned lifting Emigrants off Earth last turn (version 0.08.0), and, since version 0.09.1, the **First to a Body** windfall and its standing +1.
_Avoid_: influence budget, diplomacy pool, action points

**Research Directive**:
The share of a Faction's Research, chosen as a percentage and standing until it is changed, that goes somewhere other than the shared Tech. Read at Income before a point reaches the Tech, so what is directed **contributes nothing to the Research Lead** -- and the Lead is the only seat that picks what the table researches next. That is the price, and it is what makes the directive a decision rather than free income.
Each Faction's goes somewhere of its own: the **Custodians** enlarge the **Natural Sink**, permanently, by 0.01 ppm a point; the **Prospectors** take 0.8 **Ducats** a point; the **Arkwrights** take a **Fuel** for every five points; and the **Archivists** pay the **Archive** fund, which they have done since version 0.07.0 and which was the only version of this until 0.08.3. Fractions of a Ducat or a Fuel are carried between turns rather than floored away.
Every Faction may direct up to **half**. The **Archivists alone may direct all of it**, because their switch always sent all of it and the slider that replaced it keeps that reach.
Since version 0.08.3 **Provisional Findings** takes a threshold rather than a yes or no: it holds while at least **75%** of last turn's Research still went to the shared Tech, so a directive of 25 or less keeps the rule and anything above trades it away.
Since version 0.08.3 the table has an opinion about it: see **the shared pot**.
_Avoid_: research split, funding (that is the Archive's own word), taxing your labs, siphoning

**the shared pot**:
What every other Faction makes of how much of its Research a Faction gives the common Tech. Contribute **all** of it and every rival thinks **one point** better of you; contribute less than **85%** and every rival thinks one point worse. Between the two, nobody minds and nobody is grateful.
It is a **term**, not a deed: read afresh at every settle from what the Faction is doing now, exactly as the **Blame** term is, so it is gone the turn they contribute again and nothing is ever banked. It **does not scar a pair** -- a Faction spending its own Research on its own business has done nothing to anybody -- and it does not consume the pair's one act a turn, because it is not an act.
The reward may not lift a pair past the top of **Cordial**, the step above Neutral. Version 0.08.2 settled that a pair which never strikes an **Accord** can never rise above Neutral; this bends that by one band rather than breaking it, so generosity stops a rival resenting you and still does not make them a friend. A pair already higher by deeds is not dragged down to the ceiling.
_Avoid_: the pot (alone), research tax, tithe, generosity score

**Research**:
Points produced by Research Labs on Earth and, since version 0.06.0, by Observatories at Colonies and Space Stations, and spent only on Techs. Since version 0.08.2 two Factions holding a research agreement between them each make a tenth more. Since version 0.08.0 a place's Education Level moderates what its population adds as well as multiplying the building's own figure, so schooling applies twice to a Lab. Held outside the Stockpile; it is not one of the three resources and never buys a Facility, Module or Ship. Since version 0.05.5 a Lab in a Region nobody holds, or one under Occupation, runs itself and pays half its yield into the Tech under research for no Faction; North America and South-East Asia begin with such a Lab.
_Avoid_: science, research points, RP, fourth resource

### The turn

**Turn**:
Two calendar months of game time since version 0.05.5 (one before), and the unit the whole game runs in. The game begins in January 2030 and runs thirty-six turns, so turn 1 is January 2030, turn 2 March 2030 and the thirty-sixth November 2035; a turn is named by its first month alone, and the top bar names it. All four Factions order simultaneously against the same board, then the turn runs through its phases.
_Avoid_: round, month, tick, cycle

**Phase**:
One of the seven stages a turn runs through, always in this order: Income, Climate, Report, Orders, Event, Resolution, End.
_Avoid_: step, stage, segment

**Orders**:
The phase in which a Faction commits everything it will do that turn: building, raising Industry Level, allocating the Influence Allotment, launching, setting transits and Stances, and ordering attacks and landings. Any order can be cancelled and fully refunded until End Turn is pressed.
_Avoid_: commands, moves, actions, player phase

**End Turn**:
The commitment. Pressing it closes the Orders phase, after which the Event card is drawn and nothing can be taken back. Since version 0.08.6 it is a **sun** -- a shaded disc with sunspots, the words beneath it, the Enter key on its hover -- that dims to embers while a Tech pick is owed and, since version 0.09.0, while a **Choice Card** drawn at the head of the turn is still unanswered -- which is the one thing that blocks End Turn before any order is given, the Tech pick and the card being the only two that block it at all; since version 0.08.7 it stands alone in its own column at the right of the Command Cluster, at the bottom.
_Avoid_: submit, confirm, next turn

**Resolution**:
The phase in which the turn actually happens: transits advance, arrivals land, Battles resolve, Occupation counts down, control transfers, builds complete, repairs finish, and Colony Ships that unload found Colonies.
_Avoid_: processing, execution, upkeep phase

**Report**:
The phase that opens a turn for the player, and the dated dispatch it shows. It opens with a headline: the most serious thing that happened, chosen by a fixed order of severity. Everything else is grouped under four headings, In space, On Earth, The climate and Your works, and an empty heading is left out. Every line that is about somewhere is a way there. It ends with what each rival Faction did, told in plain sentences of what the board could see. It speaks only of what is worth a line: since version 0.07.6 migration is reported **once per Region, in net**, and only where the net is worth at least half a person.
_Avoid_: summary, news, digest

**Tutorial**:
The guided first turns, since version 0.07.5, asked for by the `Play Tutorial` tick at the foot of the Custodians' card on the Faction screen -- since version 0.07.6, which retired the title screen's button -- and played as the Custodians from a start the player chooses as in any other game. It is an ordinary game on an ordinary board: nothing is forced and nothing is checked, and a **tutorial note** -- drawn as a Moment is drawn -- opens each of the first six turns (five until version 0.08.6, which put a Habitat on the ISS before the recruit) to say what that turn is for. The last note says so and the tutorial ends itself; the game carries on with nothing thrown away. The game does not remember it has been played. Its words live in `assets/data/tutorial.toml`.
_Avoid_: walkthrough, onboarding, training mission, guided mode

**Moment**:
A short modal that stops the turn before the Report, for one sentence and one number: a Colony founded, a Break or the sea rising, a Battle that cost a unit (since version 0.08.5 any unit, an Army included, named), a place taken by force with buildings burned (its own kind since 0.08.5), a place changing hands, a Tech completed, Antarctica opening, the Archive finished, Colonists lost in transit, or, since version 0.08.4, **a rival closing on its Victory Condition** -- three quarters of the way there, or one part met with the other short, once each, and never the player's own seat. At most two a turn, the most serious first, and every kind can be switched off.
_Avoid_: popup, alert, notification, cutscene, interruption

**Spectator**:
Someone watching a game they hold no seat in. The computer plays all four Factions and the interface gives no orders at all; in return every Faction's board, card and Standing is open to be read. End Turn advances one turn, and the turns can be set to run on their own.
_Avoid_: observer, replay, demo

**Save**:
A turn start written to a file, holding everything about one game: the same seed, the same deck, the same board. The game writes one of its own every three turns and when the game ends, keeping the last three of a game, and the player may write one at any turn start where nothing has been ordered yet. Saves are loaded from the title screen, and one written by another version of the rules is refused rather than brought forward.
_Avoid_: checkpoint, snapshot, savegame

### Combat

**Battle**:
A melee at one place in which every Faction present is hostile to every other: a Region, a Colony, or, since version 0.09.0, one **orbit** of a Body, where before it was the whole Body, so two fights in two orbits of one Body are two Battles with two records and two marks. It is resolved automatically in rounds during end-of-turn processing and is always finished inside the turn. Each round a party's chance to hit is its share of the total strength present, and its hits are spread across the other parties in proportion to theirs. Since version 0.08.8 the escort takes the fire: while a party has a warship still engaged, a hit on that party lands on one of its warships, and an unarmed hull, a Colony Ship or a Carrier, is neither struck nor pursued until no warship of its remains engaged; a Battery counts as armed, and every Army is. A Battle is three rounds; since version 0.08.8 a Battle in orbit rolls for a hit once a round for every warship and Battery still engaged across every side, never fewer than three, where before it rolled a flat three however many hulls stood in it, so nine hits was any Battle's ceiling and a Battleship could not die in a turn; a Battle on the ground still rolls three. Since version 0.08.5 a Battle on Earth pollutes: its hits are Emissions at the next Climate phase, and Blame for those who landed them. Until version 0.08.6 a quarter of the buildings at the place rolled to burn after every ground Battle; a Battle itself burns nothing now, and only a place taken by an Occupation that ran its three turns rolls. Since version 0.09.0 the odds a player is quoted before opening one -- on an attack button's hover, on a Ship stack's card, and in the Battle record afterwards -- are **the whole Battle's**: the chance of holding the field when it is over, which is the test an Occupation makes. It is measured rather than derived, the Battle fought a thousand times over on a copy of the board from a seed of its own, so it never moves and asking for it never spends one of the game's dice. Before that the figure was the first exchange's share of the strength, which on a measured board read 19.8% where the honest figure was 1%. The computer's own bar is still the first exchange's, so the threat line on a Region's card, which compares against that bar, still says first exchange and means it. Since version 0.09.1 a Battle in orbit **costs Fuel**: every Ship named in it pays two out of its own Tank, once, whichever side it is on and whether or not it opened the fight, and a Tank with less than that in it is simply emptied. A Battery pays nothing, having no Tank, and a Battle on the ground costs nothing, Armies having none. A hull whose Tank held less than the charge when it was taken fights that Battle at **half its strength**, rounded down, so a fleet that cannot refuel is a fleet that fights worse every turn; the charge and the penalty are read off one reading of the Tanks, so a hull that could just pay fights whole in the Battle that empties it. The Battle's record says what the fight cost in Fuel and names any hull that fought dry.
_Avoid_: fight, engagement, skirmish, encounter

**Stance**:
The one order a stack carries into end-of-turn processing: Attack, Hold, Intercept (Ships only, engaging arrivals before they unload), Blockade (Ships only, since version 0.08.5), Evade, or, for Armies since version 0.08.6, **Dig In**. A stance persists until it is changed; Hold does nothing of its own. Since version 0.08.7 each stance has one sentence, kept in one place and read by the stance row's label hovers and the roster rows: Attack strikes at the place it is sent to, or fights where it stands; Hold stands and fights where it is; Evade avoids battle where it can, an even chance to slip away before the first exchange; Intercept fights what arrives this turn, before it can land, and since version 0.09.0 only what arrives in its own orbit and since 0.09.1 only while its own Tank holds the two Fuel a Battle costs; Blockade shuts the orbit it is given in to every other Faction; Dig In fights two stronger in defence and never disengages. A Ship's roster row carries its stance word as an Army's has.
_Avoid_: order, mode, posture, aggression setting

**Dig In**:
An Army's third live stance, since version 0.08.6. Dug in, an Army fights two stronger while it defends and never rolls to disengage; its hit points do not follow the bonus. It cannot march or board a Carrier until its stance has been changed and the turn has passed, so digging out costs a turn. It takes effect at the Resolution of the turn it is ordered, like every stance. A neutral Region's own Armies -- its Standing Army and its Levy -- are always dug in, since nobody can order them; a held Region's take the holder's order. The computer digs in where a rival's Army stands next door and it has no cause to attack, and wherever it occupies. Shown as a word on the stance row, a trench line under the Army's shield on the map, and a Report line the turn it happens.
_Avoid_: entrench, fortify, defend, garrison mode

**Strength**:
How hard a unit hits. A Ship's type sets it, and Hardened Hulls raises it for every Ship. An Army's is the Region's since version 0.08.6: a Standing Army's is its Region's Industry Level plus one plus its armed steps, read live; a raised Army's the same figure at the raise, fixed; a Colony's the average of its Faction's Regions. What an Army defends at is more: its Region's Constabulary and calm for a Standing Army, and Dig In's two for any Army dug in. Since version 0.09.1 a Ship that goes into a Battle in orbit with less Fuel in its Tank than the Battle costs fights at **half** its strength, rounded down, for that Battle and for every Battle it fights dry; a hull with no strength to begin with, a Colony Ship, a Carrier or a Missile Carrier, loses nothing by it.
_Avoid_: attack, power, combat value

**Hit Points**:
How much damage a unit can take before it is destroyed with everything it carries. A Ship's type sets it; an Army's equal its strength since version 0.08.6 -- live for a Standing Army, fixed at the raise for a raised or a Colony's -- and never its defence. Damage persists until repaired at a Shipyard or Launch Site (Ships) or in a controlled Region or a Colony with a Barracks (Armies).
_Avoid_: health, HP, hull, morale

**Pursuit**:
A unit's ability to catch an enemy unit that disengages, forcing it to take one more round of fire. Frigates have the most; Colony Ships none.
_Avoid_: speed, chase, initiative

**Disengage**:
A damaged unit's attempt to leave a Battle, more likely the more damage it carries: after every round, its damage over its hit points over a figure in the data, three since version 0.08.6 and two before, so a unit at half its hit points leaves one time in six. A unit that disengages and is not caught survives and cannot be attacked again that turn. Evade is a chosen flat half before the first round, not this roll.
_Avoid_: retreat, rout, flee, break

**Battle Report**:
The account of every Battle from the last end-of-turn processing, read at the start of the next turn. Since version 0.08.5 every Battle is also a line of the Report at its place, ranked with a Ship destroyed when a unit died and unranked when nobody lost one, so a skirmish never reads over a Break; its parties name every unit and what it took, and an attacker's odds as the attack button's hover quoted them (on the button's face until version 0.08.7) -- the first exchange's share of the strength until version 0.09.0 and the whole Battle's chance of holding the field since; and since version 0.08.7 the map draws it for the one turn the record lives: since version 0.08.8 as the **Battle mark**, crossed blades on a disc in the aggressor's colour beside the label of the Region, the Colony or, in orbit, the orbit it was fought in -- the Body itself until version 0.09.0, which made a Battle a thing of one orbit, so two fights at one Body are two records and two marks (a ring round a Region's or Colony's label and nothing in orbit before that), its hover reading the record and its click opening the Report; a row of the Body's orbit band and a *Battles last turn* list on the Solar System Map's page say the same; and a red pip on any shield whose stack carries damage; and an Army destroyed is a line by name, as a Ship has been.
_Avoid_: combat log, after-action report

**Orbital Control**:
Held in a Body's **low orbit**, since version 0.09.0, by a Faction that has a Frigate or Battleship there with no enemy warship still engaged; before then it was held at the Body as a whole, in any orbit. It belongs to low orbit because low orbit is the lane to the ground, and the ground is all Control governs. Since version 0.07.0 it governs the GROUND alone: Armies and Colonists land freely unless a rival holds it outright, so an orbit two Factions contest shuts out neither. What a station suffers is a Blockade, which is a different thing; but since version 0.08.5 a Colony on the ground is starved -- every Module making nothing and paying its upkeep -- while one rival holds Orbital Control of its Body outright and has a stack there ordered to Blockade. A contested orbit starves nobody, as it lands nobody. Since version 0.08.8 a Battery denies it, and since version 0.09.0 a Battery covers **its own orbit alone**: a ground Colony's Battery denies low orbit, so a Colony that arms itself keeps a fleet off the ground and must be shot down first; a station's Battery denies its station's orbit, so it shields the station rather than the whole sky. The Battery's owner gains no Control by it. Since version 0.09.1 a warship holds it only while its Tank holds the two Fuel a Battle costs: a Faction holds an orbit exactly as long as it could still fight for it, and a dry rival is no rival for the test either, so a fleet that spends its last Fuel winning a Battle loses the orbit it just won to the next hull that arrives.
_Avoid_: blockade, orbital supremacy, space superiority

**Blockade**:
What a warship stack does when it is ORDERED to, since version 0.08.5, in the orbit it sits in: a Ship chooses the Slot it arrives into when its leg is ordered, before it can see who will be there, and from version 0.07.0 to 0.08.4 a Frigate or Battleship sitting in a Slot blockaded it by presence alone; now a Blockade is a stance, chosen against the rival station standing there, and a stack on any other stance blockades nothing. A blockading stack shuts that orbit to every other Faction: nobody may unload Colonists or Armies into the station standing there, nobody may refuel from it, and an empty Slot under blockade cannot be built into. Since version 0.08.5 the station standing there is starved as well: every Module makes nothing and still pays its upkeep, its Relay gives no Allotment, nobody dies and nothing is destroyed, and each turn of it is an offence at weight 1 against the station's holder, read live at each Income. It never reaches another orbit at the same Body and never shuts a Faction out of a place no warship is sitting on. The ground is starved instead, while one rival holds Orbital Control outright and has a stack on Blockade **in low orbit** -- since version 0.09.0 in low orbit and not merely somewhere at the Body, because low orbit is the lane the supplies come through. Since version 0.08.8 a Blockade shuts nothing against a Faction with a working Battery covering that orbit: the station is not shut and not starved, and the computer does not order one there. Since version 0.09.1 a blockading warship must hold the two Fuel a Battle costs, as it must to hold Orbital Control: a dry hull shuts nothing, and the order is refused while no warship of the Faction's there can pay for a fight.
_Avoid_: siege, embargo, interdiction, orbital control

**Bombard**:
An Orders-phase order, since version 0.08.8, for a Battleship at a Body other than Earth: it strikes a rival's Colony there. Since version 0.09.0 **the orbit it is in is the orbit it must hold**: a Colony on the ground is struck from low orbit by a Faction holding Orbital Control there outright, and a Space Station from its own orbit, with no rival warship and no rival working Battery in it. It is also the second action against a station a fleet has beaten, since an Attack on a station that cannot fight back does nothing beyond the Battle. Resolved after the orbital Battles, one Module of the Colony drawn at random (the Core Module and the Archive never among them) rolls the destruction chance a taking uses; when a Habitat burns, the Colonists beyond the room left die with it, which is the one way a Bombard kills people. Each Bombard is an offence at rung 3 against the Colony's holder, as opening a Battle is, and breaks a non-aggression Accord if one stands; a burned building's war Emissions are the bombarder's. One Bombard per Battleship a turn, never over Earth, where the opening stations stand. It is a Battle's act: a Report line hit or miss, a Moment and the Battle mark when anything burned. A standing Battery, which denies every rival Orbital Control in the orbit it covers, is the shield against it.
_Avoid_: orbital strike, shelling, siege, raid

**Launch**:
The Orders-phase order, since version 0.09.1, that fires a Missile Carrier's Warhead at a Region, a ground Colony or a Space Station a rival directs. It needs what a Bombard needs -- the hull in the orbit that touches the place and holding that orbit outright -- but unlike a Bombard it is lawful over **Earth**, and a Region is a lawful target: a nuke at home is the point of the weapon. Free to order, since the price was paid at the build and is paid again at the Rearm. Resolved after the orbital Battles, **every** building at the place rolls a destruction chance far above a taking's, the Core Module and the Archive spared, so a place is gutted and never removed from the board; between two fifths and three fifths of its people die; and at a Region the Standing Army is destroyed and the Industry Level falls by one, never below the board the Region started on and raisable again by the ordinary order. It is an Offence at rung 4 against the holder. On Earth alone it also fouls the air and raises the Natural Sink for good; a Launch off Earth does neither.
_Avoid_: nuke (in the interface), missile strike, bombard, first strike

**Occupation**:
The state of a Region or Colony whose defenders were beaten by an Army. The occupier chooses build orders but does not direct its Armies; control transfers after the lesser of three turns or the population being Pacified. Since version 0.08.6 the two ends differ: a transfer by the three-turn clock rolls a quarter of the buildings to burn, a transfer by Pacified takes the place **whole**, and either fires the Moment for a place taken by force. An Occupation **breaks** the moment the occupier has no Army at the place, whether the last one marched off, was lifted or was destroyed, and since version 0.08.6 a break costs: the place hands back to its previous holder at +2 Unrest, the occupier takes a rung-2 Offence from that holder (a neutral charges nobody), and the Standing the Occupation had banked for the occupier is wiped. The march itself stays legal: an occupier may leave, and pays. Since version 0.08.8 it is held by the same presence that begins it: an Army of the occupier at the place that did not escape, so an Army that ran holds nothing.
_Avoid_: conquest, annexation, capture

**Offence**:
An act against a rival that costs Relations, weighed on a ladder: **1** for Influence spent on a place the rival holds (once per place per turn), a Smear, an Agitate, or a turn of Blockade; **2**, since version 0.08.6, for an Occupation of the rival's place that broke; **3** for opening a Battle against them, bombarding them or breaking an Accord with them; and **4**, since version 0.09.1, for a Launch at a place they hold, the heaviest rung there is. A turn charges the sum of every offence in it, to a cap, and an offence against a non-aggression partner breaks the Accord as well. The rival's Report names who paid.
_Avoid_: crime, aggression score, penalty, grievance

**Pacified**:
An occupied population whose occupier's Influence, gained automatically each turn of Occupation, has passed the place's threshold. The gain is halved while the state's Unrest is past its first threshold. Control transfers at the moment the threshold is passed, and since version 0.08.6 the place is taken whole: nothing rolls to burn. Pacified is a condition of the place's people, not of any building.
_Avoid_: subdued, loyal, converted

### Winning

**Victory Condition**:
What one Faction must achieve to win. Since version 0.08.0 the Archivists' second part counts Colonists UPLOADED into the Archive rather than Colonists living beside it, which makes that half monotonic: an uploaded Colonist cannot be lost to a raid, a crowding death or a handover. Each Faction has its own, and meeting it in an End phase wins the game at once; since version 0.06.0 it is not met until the Faction's Victory gate, a Tech of its own on the Tech Tree, stands, though every part of it accrues before that; if more than one seat meets it in the same phase, the larger margin over its own bar wins and an exact tie is a draw. If no Faction has met its condition by the end of the last turn, the seats are ranked by the percentage of their own condition, then by Colonists off Earth, then by Colonies held.
_Avoid_: win condition, goal, objective, victory points

**Extraction Total**:
Retired in version 0.05.5 for the Venture Capital Fund. It was the Prospectors' measure: all the Materials and Fuel their Mines, Refineries and Factories had produced across the whole game, counted cumulatively and never spent down.
_Avoid_: extraction total, production score, output total, wealth

**Venture Capital Fund**:
The Prospectors' own pool beside the Stockpile, and their measure since version 0.05.5. **Since version 0.08.3 it holds Ducats, and since version 0.08.4 2500 of them is the first part of their Victory Condition** (2000 in 0.08.3); it held Materials before that, with a bar of 750 and then 1000.
On any turn they set the share of their **Ducat income** that goes into it at Income, from nothing to four fifths in whole percents on a slider (since version 0.08.4; in tenths before), taken before they can spend a coin of it; Ducats got by selling are not income and never reach it. A **withdrawal** takes Ducats back out at a tenth's loss. It is not on the top bar (a figure stood there beside Materials from version 0.05.5 to 0.08.6; version 0.08.7 cut it, since no other Faction's fund is on the bar): its figures are the Victory window's.
The change is more than a change of units. A share of Materials output skimmed a resource they stockpile anyway, so the hoard cost them little; Ducats are what everybody spends on Influence, Relief, Resettle and repairs, so the Fund now competes with the Faction's whole economy and the share is a **decision taken every turn** — bank it or spend it — which is what a venture fund is. The bar of 2000 was set by measurement rather than by converting the old one at the market rate, leaving the median long game just across the line, exactly as 1000 Materials did; 2500 is a deliberate stretch past it, taken in version 0.08.4 with other changes expected to raise their income.
_Avoid_: the pool, savings, treasury, war chest, bank

**Stabilization**:
The Custodians' measure: net Emissions held under the Natural Sink, counting every Scrubber, for a run of consecutive turns. One turn over the Sink resets the run.
_Avoid_: carbon neutral, balance, equilibrium

**Off-world Presence**:
The number of Colonists living away from Earth, required by the Custodians' and the Prospectors' Victory Conditions. No Faction can win on Earth alone. Colonists in Antarctica are still on Earth; since version 0.06.0 those on a station over Earth are off it, as is everything else that asks "off Earth": the Archive's place and the Archivists' Research.
_Avoid_: population off Earth, colony size, settlers

**Chronicle**:
The page the game ends on, since version 0.09.0, reached by a **Chronicle** button on the game-over box, which keeps its Title screen and Quit. While it stands neither the board nor the box is drawn. It is a summary and not a narrative: the designer was offered three dated records of the game with the lift stated and named this instead. It carries the four Factions **ranked as the End phase ranks them** -- by score, then Colonists off Earth, then Colonies held -- where the box had listed them in seat order since the First Playable, with the tiebreak named on every row one decided and a sentence each on how that Faction ended and what held it back; a table of nine figures a Faction, every one already kept by the game (Materials, Fuel, Energy and Ducats at the end, Colonists off Earth, Regions held, Colonies and stations held together, Research made over the game, and Blame in ppm with its share); and the population and the temperature charts, the top bar's own, side by side with the in-game date on their axes. Nothing in it is new state: the game keeps no dated history of control or of cards, and the Chronicle does not invent one.
_Avoid_: history, timeline, narrative, log, after-action report, epilogue

### Climate

**CO2 Stock**:
The amount of CO2-equivalent in Earth's atmosphere, in parts per million. Emissions add to it each turn; the Natural Sink takes a little away.
_Avoid_: pollution, carbon level, warming points

**Temperature**:
Degrees above pre-industrial. It follows the CO2 Stock with a lag of one to two turns and is what actually harms population and drives Events.
_Avoid_: heat, warming percentage

**Emissions**:
The CO2-equivalent a source adds to the CO2 Stock in a turn. Every source has its own figure and the Climate Panel shows them one by one before the sum; methane-heavy sources carry a heavier weight. Since version 0.05 a Region's people emit more the more built-up the state is, and a Leapfrog lowers that state's figure for good. Since version 0.08.5 war is a source: every hit landed in a Battle on Earth or in Earth orbit, and every building burned in the rolls after a ground Battle or a taking, puts ppm in the air at the next Climate phase, worn as Blame by whoever landed the hit or made the taking, nobody's for a neutral Region's own Army, and counted against a Stabilization run. A Battle at any other Body fouls nobody's air.
_Avoid_: output, carbon, footprint

**Emissions history**:
Since version 0.07.4, the record the engine keeps of every Climate phase -- the breakdown by source, the CO2 Stock, the Temperature and the Breaks that fired -- saved with the game, and the chart drawn from it: what the world emitted, what the Natural Sink and the Scrubbers removed, and the net between them, turn by turn against a zero line, the Breaks ticked red on the turn axis. Drawn small on the top bar's Emissions hover and wide on the Climate Panel under the by-source list. Its siblings are the **Temperature history**, which draws the Temperature turn by turn on the top bar's Temperature hover, on the data's own range with the Breaks' Temperatures as faint lines; and, since version 0.07.5, the **population history**, which draws Earth's people and space's on two scales of their own, since one axis would lay the space line flat; and, since version 0.08.4, the **Victory history**, one Faction's progress and Blame share turn by turn on the Faction window. Since version 0.07.5 every chart's time axis carries the in-game date and never the turn number, and the record carries the two population figures, written after the Climate phase has settled them.
_Avoid_: emissions total, emissions log, graph, chart (as names)

**Victory history**:
Since version 0.08.4, the record each Faction keeps of every Climate phase -- how far along its Victory Condition it stands, the lower of its two parts' fractions, and its share of the four Factions' Blame -- saved with the game, and the chart drawn from it at the head of the Faction window's page (under the Victory progress block until version 0.08.7 cut that block): the progress in the Faction's colour, the Blame share on a scale of its own with the fair quarter marked, the in-game date along the foot, and ticked there a Break in red, Antarctica's opening in blue, and in white the turn the Faction's gate Tech was done and, for the Archivists, the Archive completed. Every Faction's page carries its own, the player's included. The first history the game keeps per Faction rather than for the world.
_Avoid_: progress graph, score chart, victory log

**Natural Sink**:
The amount of CO2 the oceans and forests remove from the CO2 Stock every turn. Net emissions below it stabilize the stock. Every Scrubber standing and online enlarges it while it stands; the Custodians' Research Directive can add to it for good; and since version 0.09.1 a Launch on Earth raises it for good as well, the soot of a nuke at home dimming the sky. Since version 0.05 a Break can weaken it for good, and since version 0.09.1 that Break **takes an amount off** it rather than setting it to a figure, so a Sink somebody has raised keeps what it was given.
_Avoid_: absorption, offset, carbon capture

**Sea Level**:
How far the oceans have risen with the Temperature. It is drawn on the globe as a band of drowned land along every coast, widening with each threshold, and at each of its thresholds it permanently takes Coastal Slots from every Region, as many as the state's Coastal Exposure and never more than it has left, and then, since version 0.08.5, turns one Inland Slot coastal, wall or no wall, so a coast never runs out. It takes nothing else and takes nothing from a state with no Coastal Slots left that rise, but it reaches every state that has a coast at all.
_Avoid_: flooding, water line, ocean rise

**Break**:
A Temperature at which a permanent change to the world fires once, the first time the Temperature stands at or above it. Five since version 0.05: the reefs die, the permafrost thaws, the Natural Sink weakens, the ice sheets go and the Amazon dies back. Since version 0.09.1 the Sink Weakens **subtracts** from the Natural Sink instead of assigning it a figure; on an untouched world it lands where it always did, but it no longer erases whatever had raised the Sink first. Nothing undoes a Break, and the Report says it happened rather than that it is coming.
_Avoid_: tipping point, threshold, trigger

**Committed Warming**:
The Temperature the CO2 Stock as it stands will deliver once the lag has caught up. It is what the world has already bought, whatever anybody builds or stops building.
_Avoid_: locked-in warming, pipeline, inertia, baked in

**Last Turn**:
The latest turn on which cutting net Emissions to zero from that turn onward still keeps the Temperature under the Collapse Line by the last turn, counting every Break the world would cross on the way. The Climate Panel names it, or says that cuts alone no longer avoid Collapse, or that Collapse is not reached on this path.
_Avoid_: deadline, point of no return, countdown

**Collapse Line**:
The one Temperature at which the game ends with nobody winning, unless a Faction had already met its Victory Condition in an earlier End phase. Every other effect of Temperature is continuous; this is the only line.
_Avoid_: threshold, tipping point, game over temperature

**Climate Panel**:
The screen showing the CO2 Stock, the Temperature and where it is heading, a Temperature bar notched with every Break, every Sea Level threshold and Antarctica's opening, this turn's Emissions by source, the sink and the net, since version 0.07.4 the Emissions history beneath them, the penalties in force, the Committed Warming, the Last Turn, and a projection to the last turn.
_Avoid_: warming meter, climate HUD
