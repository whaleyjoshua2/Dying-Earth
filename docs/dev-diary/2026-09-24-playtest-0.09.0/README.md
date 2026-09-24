# Four playtests of version 0.09.0

Four agents, one per Faction, played version 0.09.0 headlessly through `engine/examples/play.rs` on
2026-09-24 at `abf0cb5`. Each played three games in persona stopped at **turn 20** and one short
**common gamer** session of about ten turns: sixteen sessions, about 280 turns. None ended early.

| Faction | hardcore 4X | casual | colonisation fantasy | common gamer |
|---|---|---|---|---|
| Custodians | seed 101, t20 | 202, t20 | 303, t20 | 404, t10 |
| Prospectors | 11, t20 | 27, t20 | 43, t20 | 5, t10 |
| Arkwrights | 4401, t20 | 7712, t20 | 9903, t20 | 5150, t10 |
| Archivists | 101, t20 | 202, t20 | 303, t20 | 404, t10 |

**How to weigh this.** Every session stopped at turn 20 of 36, so nothing here is evidence about the
last third of a game. Two agents shared a scratchpad path and one overwrote the other's order file
mid-run, which cost a few turns of one session. Everything marked **verified** was checked against
the code or the data afterwards by the driving session; everything else is a player's impression,
which is what a playtest is for.

**Contents.** [Verified defects](#verified-defects) · [What all four found](#what-all-four-found) ·
[What each persona found](#what-each-persona-found) · [Why each Faction wins or does not](#why-each-faction-wins-or-does-not) ·
[Every recommendation](#every-recommendation-by-faction) · [Exploits](#exploits) · [What worked](#what-worked)

---

## Verified defects

Four claims were checked in the tree and are true.

**1. A Shipyard shut for want of Energy refuses with "no Shipyard here".** All four playtesters hit
this independently. `engine/src/orders.rs:1059` tests `m.working()` and the message names absence.
The player is told the wrong rule, in the version whose headline improvement is that a refusal names
the right one. The lift refusal has the same shape.

**2. Colonists aboard a Ship count for nobody.** `Game::off_world_colonists`
(`engine/src/state.rs:2856`) sums Colonists in Colonies only. The Victory bar therefore **falls** the
moment a player loads a Colony Ship: 28 to 16 in one Arkwright session, 8 to 4 in a Prospector one,
unannounced. The cheapest way to protect the bar is never to fly, in a game about leaving Earth.

**3. The computer Archivists are never told to research their own path.** `assets/data/ai.toml:398`
lists `public_science, efficient_grids, coastal_engineering, green_consensus, civil_defense`. Not one
is on the path to The Upload, which is their Victory Condition. Every other Faction's list names its
own gate's parents. This is a one-line data fix and the best explanation on file for **0 wins in 80**.

**4. Greenwash is fifty times cheaper than the card that does the same thing.** `[greenwash]` in
`influence.toml` removes **2 ppm of Blame for 1 Influence and 1 Ducat**. The Carbon Offset Scheme
choice card asks **50 Ducats for 2 ppm**. Both are live in the same game, and the card is a decision
no informed player takes.

---

## What all four found

**Energy runs out in silence and switches the game off.** Every agent, every Faction. Buildings go
dark with no Report line; the only sign is an `(offline)` tag inside a list. One Custodian session
lost three Scrubbers this way and the Natural Sink fell 37 to 28 ppm, putting the world back on
course to collapse, with nothing said. A beginner had seven of eight Facilities dark by turn 5.

**You lose your home Region with no warning.** Rivals' Standing is invisible from the board, and the
price to hold a place rises as a rival climbs. The Prospectors lost their capital with its Launch
Site and Investment Bank on turn 12; the Archivists lost their home in three sessions of four. The
mirror is as bad: three agents poured Influence for turns into Regions they already held because
nothing said so.

**The research tree is a lottery that decides the game.** Only the Research Lead picks, from three
drawn at random. The Lead's **own gate** is protected; its **prerequisites are not**. Measured chain
lengths: Custodians 98 Research across three Techs, Prospectors 130 across four, Arkwrights and
Archivists **148 across five** — and those two win 1 game in 160 between them.

**The opening tells every Faction the wrong Victory Condition.** The turn-1 note says "get twelve
Colonists off Earth" to everyone. That is the Arkwrights' condition.

---

## What each persona found

Each persona found a class of problem the others walked past. This is the cut worth reading if you
are deciding what to fix first.

### The common gamer: everything about first contact

Ten turns each, and all of it is the first ten minutes of a new player's life.

- **They were the only one to act on the turn-1 note.** The Prospector common gamer finished ten
  turns with **22 of 12 Colonists off Earth and a score of 0.00**, having done exactly what the game
  told them, twice over, while the 2,500 Ducats their Faction actually needs were never mentioned.
  Only a first-timer trusts that text literally.
- **They painted themselves into a corner by turn 4**, filling all seven of India's build slots with
  a Factory, a Bank and a Lab before learning a Power Plant was needed. The only escape is
  Decommission, which a beginner will not find.
- **A card you can only refuse still stops the turn.** Asked for 30 Ducats holding 3, a newcomer
  reads that as the game being broken rather than as a rule.
- **They hoarded**: 126 Ducats by turn 10 with nothing bought, and 32 Pioneers stuck on Earth with
  nowhere to send them.

### The casual player: the boredom, because only they reported it as boredom

- **They named the exact turn they stopped caring.** Turns 9 to 20 are the same five order lines and
  the only live decision is which Region to buy next. The hardcore player optimised through that
  stretch; the fantasy player ignored it. Both casual Prospector games died there.
- **The Venture Fund slider, which decides their whole Faction, sits at zero by default** and took
  six turns to find. A casual player will not go hunting for it.
- **The mandatory Tech pick interrupts almost every turn**, usually among Techs irrelevant to them.
- **"Archive fund: 20 of a cap of 20" beside "The Archive: 20/80" reads as a bug**, not as a rule.
- **The Colonist half of the Victory Condition is a repeat-click** finished by turn 6, after which
  half the condition cannot be touched again.

### The hardcore player: the systems failures, because they measured

- **Stabilization pays nothing until it is met and is never quantified.** Only someone counting
  notices that cutting world net emissions from +43 to +21, with eleven Scrubbers, four Leapfrogs
  and four mothballed countries, scores **exactly zero**. The Victory line reads `0/3` and there is
  no ppm gap on screen; worse, the figure shown on the Climate line is not the one the test uses.
- **They found the Prospector dominant engine**, by optimising until it appeared.
- **The cheapest-tech reroll and permanent gate denial**, both consequences of how the shortlist is
  drawn, which only a player gaming the draw would see.
- **Retaking a Region is an auction with a hidden target**: 150 Influence over five turns reached
  "Standing 146, 152 needed", the bar rising as fast as they climbed.
- **Conquer-and-mothball is the Custodians' only working line**, and the game documents neither the
  idea nor its price of +1 Unrest a building.
- **The 148-against-98 Research arithmetic** that explains two Factions winning one game in 160.

### The colonisation-fantasy player: the hollowness

The finding I would take most seriously, because it is about what the game is for.

- **Twelve turns, a Shipyard, a Habitat, two Solar Arrays, a Colony Ship, about 130 Materials and 60
  Fuel, to put four people on the Moon** — worth exactly what four free lift orders to a station you
  already own are worth. The colonisation is optional in a game about colonisation.
- **The game then called it "a Colony in slot 1 on the Moon"**, when the place is named Mare
  Tranquillitatis and it was slot 0.
- **A founding seats only the Core Module's flat four**, however many you carried, silently, with no
  Report line and the rest kept aboard.
- **A poor start means never leaving Earth at all.** Brazil: GDP 5, two Facilities, 2 Ducats and 4
  Materials a turn. The Colony Ship was not affordable until turn 18, and when it reached the Moon
  on turn 20 the free-slot list read `the Moon ground: none` — the Archivists had taken all four.
- They are also the persona who noticed the world has no voice at these moments.

**The pattern worth naming**: three of four agents said founding a Colony on the Moon was the best
moment in seventy turns, and the fantasy player is the one who showed it costs twelve turns and buys
nothing the game scores.

---

## Why each Faction wins or does not

**Prospectors, 31 wins in 80.** The dominant line is an engine, not a strategy: bank at the 80% cap
on turn 1, raise Industry Level in every held Region forever at their half price, put an Investment
Bank in each, and **never build a ship**. A level pays back in about five turns and appears uncapped;
one game reached Industry 12. Nothing else competes, and the Faction's other half is free.

**Custodians, 5 wins in 80.** Their condition is a **world** figure that pays nothing until met and
is never quantified. The playtester cut world net from +43 to +21 and scored zero while the
Prospectors' Fund ticked up all game.

**Arkwrights, 1 win in 80.** Reachable on paper, not in practice. 148 Research across five Techs,
four of them unprotected lottery draws, against a physical timetable that is also late: first Colony
Ship turn 10 to 12, first Colony turn 14 to 15, third Body about turn 25, gate about turn 27, with a
collapse forecast between turn 23 and 35. Mars is unreachable outside its launch window, because the
leg costs 35 Fuel and the tank holds 30.

**Archivists, 0 wins in 80.** The path is fine: played by hand, the Archive completed on turn 17, 10
of 12 were uploaded by turn 18, and the seat was leading the table with half the game to spare. What
stops them is the data defect above, plus the fact that a non-Leader Archivist may never be offered
The Upload at all, because AI seats reach a rival's gate only by a cheapest-remaining fallback and
The Upload is joint-dearest.

---

## Every recommendation, by Faction

The four lists in full, each in the order its playtester ranked it, with the persona it came from and
the size they judged it.

### From the Custodians' playtester

1. **Energy runs out silently and switches the game off.** *(all four personas; small–medium)* A
   standing warning when net Energy is negative, and a Report line naming the buildings that went
   dark. Make the refusals honest: a powered-down Shipyard says "no Shipyard here".
2. **Stabilization is a world condition, pays nothing until met, and is never quantified.**
   *(hardcore; medium, a design call)* Show the gap in ppm on the Victory line, and give the run
   partial credit, or the Custodians cannot even place in the final ranking.
3. **The Custodian gate Tech can be unreachable by luck.** *(hardcore/fantasy; medium)* Planetary
   Stewardship is rung 3 behind two others on a shared tree where only the Lead picks. Put the
   holder's gate chain on their shortlist, or print the path in the Victory panel.
4. **Rivals' Standing is invisible and the bar moves under you.** *(hardcore + casual; small–medium)*
   Show the leading rival's Standing, and flag Influence spent on a Region you already hold.
5. **Conquer-and-mothball is the Custodians' only working line, and the game hides the idea and its
   price.** *(hardcore; small)* Say the +1 Unrest cost on the order, and let the Faction text admit
   that switching Earth's industry off is their lever.
6. **The Colonist half is a repeat-click finished by turn 6.** *(casual; medium)* Raise it, or tie it
   to Bodies the way the Arkwrights' is.
7. **A card can be shown to a seat that was never asked.** *(defect; small)* `show` printed the
   question with both answers, then refused the answer: "they were not asked".
8. **A poor start is a dead start.** *(fantasy/common gamer; medium)* Warn on the start card, or give
   every start a generator.
9. **Smaller ones.** `build army` confirms "costs 25 Materials" and never mentions the million people
   or the Widgets, so the version's headline rule is invisible where it is paid. Two phrasings for
   one block ("The turn was NOT ended:" / "The turn did NOT end:"). Army rows print `seat Some(0)`
   where Ship rows print `seat 3`. `show` never prints market prices, which swing 1 to 4 in two
   turns. `transit` reports "costs free" while spending the tank. The headline can bury your own
   conquest.

### From the Prospectors' playtester

1. **The opening tells the player only half their Victory Condition.** *(common gamer; tiny change,
   biggest effect in the report)* The 2,500 Ducats are never mentioned.
2. **The Fund slider is the Faction's whole game and is invisible.** *(casual; small)* At 0% the Fund
   creeps 1 a turn against a bar of 2,500. Default it to something, or demand it on turn 1 the way
   the Tech pick is demanded.
3. **The board calls the Fund "0 Materials, banking 0% of output".** *(all; trivial)* It holds Ducats
   and banks a share of Ducat income. Two versions stale.
4. **Buildings go offline for Energy in silence, and the refusal then names the wrong rule.**
   *(fantasy; medium)* Six turns lost to an invisible cause.
5. **The Faction's gate Tech sits behind a random three-item shortlist in a shared tree.**
   *(hardcore; large)* The Extraction Charter was never offered in four games, so the Victory
   Condition was unreachable in all four. Nothing names the prerequisites or the cost.
6. **The whole board's economy dies at once around turn 14 to 16.** *(hardcore/casual; large)* Unrest
   reached 9 or 10 in every Region on the map, mine, the computer's and the neutrals'. Income fell by
   four fifths. Relief moves it 1 a turn per Region at 10 Ducats and cannot keep up. The back half of
   the game is four players poking a corpse.
7. **You can lose your capital with no warning in the order stream.** *(all; medium)*
8. **Retaking a Region is an unwinnable auction with a hidden target.** *(hardcore; medium)* Show the
   number to beat, or cap the escalation.
9. **Turns 9 to 20 are the same five lines.** *(casual; medium)* Where they stopped caring, in both
   games.
10. **Choice Cards you cannot engage with still stop the turn.** *(all; small)* Three of four cards in
    one game were ship cards for a seat with no ships; each printed "you had nothing to decide"
    without saying why, or which side was taken.
11. **The colonisation fantasy has no payoff.** *(fantasy; medium)* See the persona section above.
12. **Smaller.** `lift europe 4 15` is refused outright when the station has room for 2, rather than
    clamping. "16 of 12 Colonists" is not clamped. Turn-end blockers are reported one at a time: fix
    the owed Tech, then discover the owed card. "Market: Materials 2 Ducats each" while paying 3.4
    four turns later.

### From the Arkwrights' playtester

1. **Put the whole gate chain on the shortlist, not just the last rung.** *(all personas; hardcore
   named it at turn 13; large)* The one that decides the game.
2. **Cut the Arkwrights' chain, or lower it.** *(medium)* Generation Ships needing Closed-Loop
   Colonies is two rungs of tax nobody else pays twice over.
3. **A founding should seat everyone aboard, or say it will not.** *(small)* Twelve carried, four
   landed, eight silently kept aboard, no Report line.
4. **"Colonists off Earth" must not go backwards when you load a ship.** *(medium)* The two victory
   bars actively fight each other. Count Colonists in transit.
5. **Pioneers pile up with nowhere to go.** *(medium)* 126 waiting by turn 20, 60 by turn 10 for the
   beginner. Cap the muster to available berths and say so, or let Pioneers lift straight onto a Ship
   in low orbit. **The single most demoralising number on the board.**
6. **Warn before Energy shuts things.** *(small, high value)*
7. **The common gamer paints himself into a corner by turn 4.** *(medium)* Reserve a slot, or flag
   the energy balance on the Region card.
8. **Turn-1 cards are dead for the Arkwrights.** *(small)* They open with 1 Ducat and two seeds dealt
   a card demanding 20 or 30. Give them opening Ducats, or do not deal a paying card on turn 1.
9. **An unanswered Tech pick did not block the turn, though a card does.** *(small)* Not reproduced
   by the driving session: `end_turn` does enforce the refusal, but it yields when the shortlist is
   empty, which may be what happened. Contradicts the new card rule either way.
10. **The Influence threshold jumps without warning.** *(medium)* Eleven turns of spending, then the
    bar went 40 to 127 when a rival took the place.
11. **Board and order names disagree.** *(small)* The card says *India*, the order wants *southasia*.
12. **"Occupied by the Custodians … you hold it on 47 Standing" on one line**, then "India now belongs
    to the Custodians" over a board reading neutral. *(small)*
13. **Smaller.** The headline says "slot 1 on the Moon" for slot 0. The flying table quoted a leg at
    4 turns and 17 Fuel and it cost 6 and 26, leaving the hull **stranded with an empty tank** and no
    station to refuel at. The *Distress Call* card silently froze the only ship mid-transit. The
    *Refugee Convoy* trades +0.4 population against +2.0 ppm, which nobody would take twice. A closed
    **sale** reads "you cannot pay what it asks" when what you lack is the goods. Unrest lines in the
    Report run out of order.
14. **Possible defect, unverified.** The Spaceport's "+1 Influence per Pioneer lifted" did not show;
    it may be that the Spaceport was shut for Energy, which the Report never said, which is the
    finding either way.

### From the Archivists' playtester

1. **Give the computer Archivists their own tech path.** *(hardcore/all; tiny, probably decisive)*
2. **Guarantee the Lead's gate chain, not just the gate.** *(hardcore; medium, one function)*
3. **Warn when a rival is climbing on a place you hold.** *(all four personas; medium)* Home lost in
   three sessions of four with no signal whatever.
4. **Tell the player their own Victory Condition.** *(common gamer; tiny)* It mis-taught three of
   four sessions.
5. **Explain the 20-of-80 cap.** *(casual; tiny)* Say "a quarter until the Archive stands".
6. **Alarm on negative Energy.** *(casual/common gamer; small)*
7. **Make the mandatory Tech pick less frequent.** *(casual; medium)* Stopped for a modal three-way
   choice almost every turn for twenty turns, usually among Techs irrelevant to them.
8. **Research Lab output is 5R in Europe and 2R in India** with nothing on screen explaining why.
   *(hardcore/fantasy; small text, large effect)* For the Research Faction this decides the game at
   the start screen.
9. **`build archive` help says nothing about The Upload.** *(casual; tiny)* The refusal is excellent;
   the order grammar should say it too.
10. **Cards that can only be refused should not block the turn.** *(common gamer; tiny)*
11. **Influence poured onto a Region you already hold is silently wasted.** *(casual; small)* Standing
    112 on Japan against a threshold of 30 before noticing.
12. **Defect: the Report says "set its Labs to pay the Archive fund" for the other three Factions.**
    *(tiny)* Only the Archivists have one.
13. **Defect: the founding headline names the wrong slot.** *(tiny)*
14. **Defect: "no Shipyard here" when the Shipyard is there but offline.** *(small)*
15. **Partial unload is silent.** *(small)* Six unloaded, four landed, two stayed aboard, nothing said.
16. **The headline repeats the Event verbatim**, and "Orbital Debris: The Custodians had nothing to
    decide" is an empty headline. *(tiny)*
17. **`lift` refuses rather than filling.** *(tiny)* The help says "as far as its Habitat room goes";
    it refuses the whole order instead, which reads as a lie.

---

## Exploits

Ranked by how much they break the game.

1. **The Prospector engine.** Industry Levels at half price plus an Investment Bank in every Region,
   compounding, uncapped, with ships never built.
2. **Half the Prospectors' and Custodians' Victory Condition is free.** Colonists on a station over
   Earth count as off Earth. One Habitat and free lifts put 12 of 12 on the board by turn 6 or 7 for
   about 42 Materials, with no ship and no other Body.
3. **Ducat builds bypass the whole version.** `-ducats` pays a flat twice the Materials, ignoring both
   the market price and the Widget queue that 0.09.0 was built around. At the market's ceiling a
   Scrubber's ore alone costs twice what the outright buy does. Buy buildings, never ore.
4. **Greenwash at a fiftieth of the card's price** (verified above).
5. **Conquer-and-mothball.** Take an industrial neutral by Influence and switch its Facilities off.
   You keep its Ducats, its emissions stop, and because Stabilization is a world figure you win by
   turning off other people's countries.
6. **Cheapest-tech reroll and gate denial.** The shortlist is redrawn only when a Tech completes, so
   the Lead should always pick the cheapest thing on the list to buy a fresh draw sooner. This makes
   rung-3 Techs nearly unpickable, and a seat holding the Lead can lock every rival out of their
   Victory Condition for a whole game. It is not quite free: one session was eventually forced to
   open a rival's gate because the list held nothing else.
7. **The Fund is a current account.** `draw-venture` returns 90% on demand, so "bank it or spend it"
   is really "bank it and borrow back at 10%", cheaper than what the banks pay in.
8. **Cheap Ore Offer pays you twice.** Its refuse side sets the **global** Materials price to 1 for
   two turns, so take the ore and let a rival's refusal crash the price for you as well.
9. **The Directive has one right answer.** Overflow above the fund's cap still reaches the shared
   Tech, so `directive 100` costs nothing, ever. The slider the Archivists are built around has one
   correct setting from turn one.
10. **Coach Class's drawback is not one.** A large Region grows faster than eight Pioneers a turn cost
    it, so the Arkwrights' double population cost never bites.
11. **Materials to Ducats to Fuel.** Materials sell at 1.5, Fuel buys at 4, and the prices never move
    because the computer never trades. You are alone in the market.
12. **Fuel is a faucet** for a Faction with no fleet: Refineries pay 4 to 7 a turn and it sells.
13. **`relief` has no per-region limit** where `agitate` explicitly does. Three Reliefs into one
    Region in one turn.
14. **Conscription Notice is always taken**: a free Army costing no people, which is not a question.

---

## What worked

Every agent, unprompted, named the same thing first: **the climate clock**. *"On this course the
world collapses on turn 34"*, and watching it move as the table's emissions move. A Custodian's
mothball turn moved it from 27 to 34 and they felt it.

**The choice cards are the best new thing in the build.** They arrive before orders, the turn really
will not end, `--force` will not push past it, and nobody resented the gate. *Surplus Habitats*,
*Overtime at the Yards*, *The Refugee Convoy* and *The Whistleblower* were all called real decisions.

**The Widget queue is legible.** `0:The Archive 8/12w` beside `Widgets 4 a turn` told a player exactly
when a thing would stand, and no agent ever wondered why a build was slow.

**The refusals teach the rule where they are honest**, and a bad line bounces the whole turn rather
than half-applying it. **The Breaks have real voice** — Coral Die-off, Permafrost Thaw, and above all
The Sink Weakens firing at +2.0 C, which undercuts exactly what a Custodian has been building — and
were the only moments one agent stopped optimising and read. **The whole-battle odds held up**: 63%
quoted, the field held. **The Strip Permit** is the best single turn in the game, income 25 to 46,
with the Unrest arriving later like a bill. **Industry Levels** are a satisfying compounding
investment. **Losing a capital to Influence** was a shock one player felt they had earned. **`show` is
genuinely complete**: nothing needed was missing from it, only from the game's explanations.

And three of four agents said the same thing about the same moment. Founding a Colony on the Moon, in
a game about leaving, still felt like something.
