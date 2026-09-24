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

**How to weigh this.** Every session stopped at turn 20 of 36, so nothing here is evidence about
the last third of a game. Two agents shared a scratchpad path and one overwrote the other's order
file mid-run, which cost a few turns of one session. Everything below marked **verified** was
checked against the code or the data afterwards by the driving session; everything else is a
player's impression, which is what a playtest is for.

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
lists `public_science, efficient_grids, coastal_engineering, green_consensus, civil_defense`. Not
one is on the path to The Upload, which is their Victory Condition. Every other Faction's list names
its own gate's parents. This is a one-line data fix and is the best single explanation on file for
**0 wins in 80**.

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
price to hold a place rises as a rival climbs. Reported as a shock by all four: the Prospectors lost
their capital with its Launch Site and Investment Bank on turn 12; the Archivists lost their home in
three sessions of four. The mirror is as bad: three agents poured Influence for turns into Regions
they already held because nothing said so.

**The research tree is a lottery that decides the game.** Only the Research Lead picks, from three
drawn at random. The Lead's **own gate** is protected; its **prerequisites are not**. So a Faction
whose gate sits at rung 3 must win several draws it does not control. Measured chain lengths:
Custodians 98 Research across three Techs, Prospectors 130 across four, Arkwrights and Archivists
**148 across five** — and those two win 1 game in 160 between them. Three agents independently asked
for the Lead's whole gate chain to be carried on the shortlist.

**The opening tells every Faction the wrong Victory Condition.** The turn-1 note says "get twelve
Colonists off Earth" to everyone. That is the Arkwrights' condition. A common-gamer Prospector
finished ten turns with **22 of 12 Colonists and a score of 0.00**, having done exactly what the
game asked, twice over.

---

## Why each Faction wins or does not

**Prospectors, 31 wins in 80.** The playtester found the dominant line and it is an engine, not a
strategy: bank at the 80% cap on turn 1, raise Industry Level in every held Region forever at their
half price, put an Investment Bank in each, and **never build a ship**. A level pays back in about
five turns and appears uncapped; one game reached Industry 12. Nothing else in the game competes,
and the Faction's other half is free (see the exploits).

**Custodians, 5 wins in 80.** Their condition is a **world** figure that pays nothing until it is
met and is never quantified anywhere. One agent cut world net emissions from +43 to +21 with eleven
Scrubbers, four Leapfrogs and four conquered-and-mothballed countries, and scored **exactly zero**,
while the Prospectors' Fund ticked up all game. The Victory line reads `0/3` and there is no ppm gap
on screen. Worse, the number shown on the Climate line is not the number the test uses.

**Arkwrights, 1 win in 80.** Reachable on paper, not in practice. Their gate needs 148 Research
across five Techs, four of them unprotected lottery draws, while the physical side is also late:
first Colony Ship turn 10 to 12, first Colony turn 14 to 15, third Body about turn 25, gate about
turn 27 — against a collapse forecast between turn 23 and 35. Mars is unreachable outside its launch
window because the leg costs 35 Fuel and the tank holds 30.

**Archivists, 0 wins in 80.** The path is fine: played by hand, one agent completed the Archive on
turn 17, uploaded 10 of 12 by turn 18 and was leading the table, with half the game to spare. What
stops them is the data defect above, plus the fact that a non-Leader Archivist may never be offered
The Upload at all, because AI seats reach a rival's gate only by a cheapest-remaining fallback and
The Upload is joint-dearest.

---

## Exploits, ranked by how much they break the game

1. **The Prospector engine.** Industry Levels at half price plus an Investment Bank in every Region,
   compounding, uncapped, with ships never built.
2. **Half the Prospectors' and Custodians' Victory Condition is free.** Colonists on a station over
   Earth count as off Earth. One Habitat and free lifts put 12 of 12 on the board by turn 6 or 7 for
   about 42 Materials, with no ship and no other Body. The colonisation game is optional in a game
   about colonisation.
3. **Ducat builds bypass the whole version.** `-ducats` pays a flat twice the Materials, ignoring
   both the market price and the Widget queue that version 0.09.0 was built around. At the market's
   ceiling a Scrubber's ore alone costs twice what the outright buy does.
4. **Greenwash at a fiftieth of the card's price** (verified above). The dirtiest Faction buys out of
   the only penalty for being dirty at pocket-money rates.
5. **Conquer-and-mothball.** Take an industrial neutral by Influence and switch its Facilities off.
   You keep its Ducats, its emissions stop, and because Stabilization is a world figure you win by
   turning off other people's countries. Strong, thematic, and undocumented.
6. **Cheapest-tech reroll and gate denial.** The shortlist is redrawn only when a Tech completes, so
   the Lead should always pick the cheapest thing on the list to buy a fresh draw sooner. This makes
   rung-3 Techs nearly unpickable, and a seat that holds the Lead can lock every rival out of their
   Victory Condition for a whole game.
7. **The Fund is a current account.** `draw-venture` returns 90% on demand, so "bank it or spend it"
   is really "bank it and borrow back at 10%", which is cheaper than what the banks pay in. The
   decision the design wanted does not exist.
8. **Cheap Ore Offer pays you twice.** Its refuse side sets the **global** Materials price to 1 for
   two turns, so take the ore and let a rival's refusal crash the price for you as well.
9. **Coach Class's drawback is not one.** The Arkwrights' double population cost never bites, because
   a large Region grows faster than eight Pioneers a turn cost it.
10. **`relief` has no per-region limit** where `agitate` explicitly does.

---

## What worked

Every agent, unprompted, named the same thing first: **the climate clock**. *"On this course the
world collapses on turn 34"*, and watching it move as the table's emissions move, is the best number
in the game. A Custodian's mothball turn moved it from 27 to 34 and they felt it.

**The choice cards are the best new thing in the build.** They arrive before orders, the turn really
will not end, `--force` will not push past it, and nobody resented the gate. *Surplus Habitats*,
*Overtime at the Yards*, *The Refugee Convoy* and *The Whistleblower* were all called real decisions.

**The Widget queue is legible.** `0:The Archive 8/12w` beside `Widgets 4 a turn` told a player
exactly when a thing would stand, and no agent ever wondered why a build was slow.

**The refusals teach the rule where they are honest**, and a bad line bounces the whole turn rather
than half-applying it. **The Breaks have real voice** and were the only moments one agent stopped
optimising and read. **The whole-battle odds held up**: 63% quoted, the field held. And three of four
agents said the same thing about the same moment — founding a Colony on the Moon, in a game about
leaving, still felt like something.
