# Playtest of version 0.1.0.6: four personas, sixteen games

Four independent agent players played the first 30 turns of every Faction, one game each, as
seat 0 against three computer Factions. They used the text tool `play`, built at `dec6d3b`. Each
had a different persona: a common gamer, a casual 4X player, a hardcore grand-strategy player
and a player living out a colonization fantasy. Seeds: 1101–1104, 1201–1204, 1301–1304 and
1401–1404, with the Faction order rotated per tester.

Each tester had only three files:
- the kit's README (`docs/playtest/PLAYTEST.txt`);
- the glossary (`CONTEXT.md`);
- a new `mouseovers.md`: every hover text the window builds, read fresh from the interface code at
  `dec6d3b`.

None saw the source, `docs/`, the tracker or any earlier playtest. `brief.md` is the brief they
worked from. Their turn-by-turn logs and final reports are `tester-1-notes.md` to
`tester-4-notes.md`.

Findings the window already answers are marked **[hover covers]**, so a text-tool artefact is not
mistaken for a gap in the game.

| Tester | Persona | Custodians | Prospectors | Arkwrights | Archivists | Rating |
|---|---|---|---|---|---|---|
| 1 | Common gamer | 76% at T31 (win ~T33) | 87% at T31 (win ~T32–33) | **won T26** | **won T27** | **6.5** |
| 2 | Casual 4X | 65% at T30 | 82% at T30 (win ~T34) | **won T25** | **won T27** | **6.5** |
| 3 | Hardcore grand strategy | **won T29** | **won T29** | **won T21** | **won T23** | **6** |
| 4 | Colonization fantasy | **won T27** | **won at the head of T31** | **won T23** | **won T24** | **6** |

**Twelve wins in sixteen games.** The four games without a win were all on course to win by
about T34. No computer Faction won, and none passed 52% Victory in any game. Every tester rated
the game 6–6.5; the average is 6.25, against 5.75 for 0.1.0.2.

**The four testers came to the same conclusion independently.** In a human's hands the
Arkwrights and Archivists are the *fastest* wins: T21–T27, against T27–T31+ for the Custodians
and Prospectors. Their low win rates in the sweeps belong to the **computer seats**:
- **The computer Arkwrights** had 0–7 Colonists off Earth at T23–T29 in most games. They shipped
  Pioneers to Antarctica, which doesn't count toward their win.
- **The computer Archivists** never researched The Upload. In one game (T4 G2) they filled the
  Archive to 250/250 without it.

Three testers said to fix these AIs before buffing either Faction (T1, T3, T4). T2 offered
Faction buffs as well (S8).

## 1. What grabbed them

- **The races and the firsts.** Winning the Moon race, first to Mars, Phobos and Deimos, and "the
  first child born off Earth, at Tycho". All four named these.
  - T4 called the Moon-race line, which names the losers and why, "the best line in the game".
  - The human won the Moon race at T5–T8 in every game where a tester recorded it, at least 14 of
    16.
- **Named ships** (T4: "Named ships make the fantasy").
- **A number that climbs.**
  - The Prospectors' Fund compounding (T1, T2, T4).
  - Getting the world under the Sink as the Custodians (T1, T3).
  - T1: "From T20 every turn was 'one more turn'."
- **The sea taking 6–21 buildings at a threshold.** "Spectacular theme" (T1). It is also the
  largest emissions cut in the game (T4).
- **The Custodians game**, for the hardcore player. The pack nearly bought China with six Scrubbers
  in it; "the game made me sweat" (T3).
- **The Arkwrights game**, for the colonization player: three ships in flight, Mars and Venus
  settled the same turn. "The game this could be for 50 turns" (T4).
- **Taking a rival's home Region by Influence.** The Prospectors' Europe fell in one turn (T1) or
  over a five-turn visible duel (T4).

## 2. What lost them

1. **Rules that bite without warning at the order** (all four testers, nearly every game).
   - **The Fuel trap.** A Colony Ship build or a refuel empties the Fuel Stockpile, and every
     Power Plant runs at a quarter at the next Income. T4 hit it 4 times in 4 games, T3 5 times,
     T2 in 3 of 4 games, T1 twice. [hover covers, partly: the window's Energy figure warns of a
     Shortfall; T4 couldn't tell whether that forecast counts Fuel-starved plants.]
   - **Founding takes 4 Colonists.** One Ship flew 5 turns to Mars with 3 aboard, couldn't land,
     and lost first-to-Mars (T2).
   - **Founding a Colony seats 4 and leaves the rest aboard** (T3: every game; T4).
   - **Crowded Colonists die silently.**
   - **A Capital needs one slot left open** (T1).
   - **4 uploads a turn** (T1, T2).
2. **Offers you can't see.** Accord and Trade offers block End Turn, but `show` never prints
   their terms (all four). T1 accepted two of them blind.
3. **One bad line refuses the whole order list** (T1, T2, T4). T3 adds that lines are checked
   top-down with no look-ahead: `buy fuel` after `build ship` refuses the ship.
4. **Accords lapse every 5 turns and must be renewed by hand** (T2: missed in all 4 games). The
   computer seats attacked the turn after a lapse: T2 6 times, T1 and T3 at Mexico on T11–T13.
   [hover covers, partly: in the window an expiring Accord is owed before End Turn.]
5. **The space side has no second act** (T4, T3, T1). One Moon colony and one shuttle meets every
   off-world bar by T14–T20; ships then sit idle for 10–15 turns. Mercury, Ceres and Vesta were
   never worth a ship.
6. **Research is a sprint** (T1, T3, T4). The Archivists' gate stood on T6–T8 in three testers'
   games; the whole tree was done by T25–T30 (T1).
7. **The Custodians' win feels out of their hands** (T2, T1). World emissions are set by everyone,
   and one Labour Dispute card switched off 6–7 Scrubbers for a turn (−12 to −17 Sink: T2 G3, T3
   G2).
8. **The Collapse forecast swings 15 turns.** T1 saw T40, then T26, then "not reached".
9. **Boredom spots:**
   - the Archivists' upload turns at the end;
   - the same Moon runs repeated;
   - building Modules nobody wants because a tier needs every slot built (T4);
   - renewing Accords;
   - idle Energy (268 for T2, 560 for T3) with no sink.

## 3. Exploits

| # | Exploit | Raised by | Strength | Reproduce |
|---|---|---|---|---|
| E1 | **Trade Post / Exchange scales with the colony's people**: +51 → +77 Ducats a turn on the Moon (China's whole economy: +9.6) | T2, T3, T1 | Game-defining: "every win ran through it" (T2) | Trade Post at a colony of 20+ |
| E2 | **`build module-ducats` / `facility-ducats` underpriced**: 2 Ducats per Material, no Widgets, lands next Resolution. The glossary says "market price of twice its Materials", about 6 a Material. Possibly a bug | T3, T4 | Strong: "THE build button off Earth" (T3) | `build module-ducats tycho factory` = 24.8 Ducats for 12.4 Materials (T4 G3) |
| E3 | **Scrubber spam**: 33 Materials for +3 Sink, capped only by population. China ~7, India ~8 | T4, T3 | Strong: decided T4 G1 | Take India by Influence, 2 Scrubbers a turn |
| E4 | **Fund compounding**: 1% per Bank and Exchange; 8–11% a turn with 9–15 payers | T4, T3, T1 | Strong; may be intended | Bank in every cheap Region, Exchange per Body |
| E5 | **Observatory over Earth / at L4**: 7–8 Research for ~31 Materials, against a Lab's 1–2 | T3 | Strong: "the Lab is a trap" | Observatory at Axiom/L4 |
| E6 | **Research Lead snowball**: lead every pick (+5 Influence each), own gate on T6–T8 | T1, T3, T4 | Moderate | Archivists: Lab T1, Observatory T3, pick own chain |
| E7 | **Ducats → Influence → Regions**: one turn's buy flips a Region; capitals included | T2, T1 | Medium-strong | `buy-influence 57` + allotment, China back in one turn (T2 G4) |
| E8 | **Mothball everything** as the Custodians: emissions down, no penalty beyond Unrest that Relief buys back | T1, T2 | Medium | Mothball held Factories, Refineries and Mines (T1 G4) |
| E9 | **Production Moved**: mothball an Earth Power Plant and a Moon Generator runs ×2.08 (×3 more under a Helium-3 card) | T3 | Medium | +70 Energy from one Module (T3 G2 T20) |
| E10 | **Split a colony to claim a Body**: 4 off Mars to Phobos for +20 Influence and a Body | T3 | Medium | 1-turn hop, no window |
| E11 | **The Moon race is free** for the human: no computer seat launched before T8 | all four | Medium | Shipyard → Colony Ship → Moon |

## 4. Defects the testers reported (not yet confirmed by me)

| # | Defect | Raised by |
|---|---|---|
| B1 | Raw key `[exodus_call_order]` in the Rival report | T2, T3 |
| B2 | Raw debug text for ferry routes: `Load { ship: ShipId(34), colonists: 7, from: State(SouthAsia), army: None }` | T2 |
| B3 | "The sea took your X" lines printed twice in the Report | T4 |
| B4 | "That Tech is not available yet" for a Tech already done | T1 |
| B5 | `help` says `change-orbit` costs 1 Fuel; to Earth L4 it charged 16.3 | T3 |
| B6 | An occupation that completes hands the Region back to its old owner: "(Influence), after Custodians took it (Occupation complete / Pacified)". It looped three times on India | T2 G4 |
| B7 | The EVENT line under the headline shows last turn's card | T2 |
| B8 | Order words don't match the shown names: `fissionreactor` for "Reactor", `powerplant` not "power plant", `tradepost` builds an "Exchange" and then `exchange` is refused. Refusal lists offer other Factions' unique buildings | T1, T2, T4 |
| B9 | "Next pick: Arkwrights." printed while the game demands *you* pick | T3 |
| B10 | "the Custodians's, cancellable" on a rival's build you can't cancel (wrong possessive too) | T3 |
| B11 | "Unrest from 0.5 to 0.5" lines | T4 |
| B12 | `PLAYTEST.txt` line 78 asks about the Fund "at 6,250"; the bar is 6,500 (confirmed: line 60 and `factions.toml` say 6,500) | T4 |
| B13 | The Colony Ship takes no Cheap Industry discount | T4 |

## 5. Suggestions to select from

The number of testers who raised each suggestion is in brackets. Within each heading, the order
is by how many testers converged on it, then by how much they valued it.

### a. More interesting and compelling

- **S1. The AI contests the leader's win** [4/4]. When a rival passes ~60% (or is under ~10
  turns to win), the computer seats switch to denial at that Faction's chokepoint:
  - the Archivists' Archive colony;
  - the Custodians' Scrubber Regions (Agitate, Influence);
  - the Prospectors' Bank Regions and Exchange colonies;
  - the Arkwrights' ferry colony and Mars landings.

  Today's late pile on the leader's *capital* is aimed at the wrong target: China fell on the turn
  T1 won elsewhere. T3 chose this as their extra dimension.
- **S2. A pre-End-Turn warnings list** [4/4]. One line each:
  - "Fuel 0: 5 Power Plants at a quarter";
  - "Magellan carries 3: founding needs 4";
  - "1 aboard is crowded";
  - "Accord with X lapses next turn";
  - "China contested: Arkwrights at 60 of 76".

  T2: this alone "would have prevented my top three frustrations".
- **S3. A second act in space** [T4, T3, T1]. Later off-world goals worth score or Influence: a
  Mars city of 30, a Ceres station, a self-sufficient colony. Or put distance into the wins
  ("20 Colonists, 8 beyond the Moon").
- **S4. Slow research** [T1, T3, T4]. Tech cost scales with the turn, or rung 4 costs 150+, so
  gates land around T20–30, not T6–T8.
- **S5. Show hidden trade-offs** [T3]. On a Region: "Taking it starts its idle Facilities: +N
  ppm". The controller's Emissions multiplier on the people line.
- **S6. Earth's collapse as a chapter, not game over** [T4]. At +3.0 C Earth goes dark, and
  Factions with people off Earth play on for N turns.
- **S7. A use for mid-game Influence and idle Energy** [T3, T2]: Leapfrogs bought with Influence,
  Influence that idles a neutral Region's Facilities, Energy sold at half, an Energy-to-Fuel
  station.

### b. Balance: bolstering the Arkwrights' and Archivists' wins

All four testers found the human Arkwrights and Archivists the *fastest* wins, so these are
mainly AI fixes. One is Faction-side, from T2, and one is a tempo change.

- **S8. Computer Arkwrights launch and leave** [4/4].
  - No Pioneers to Antarctica.
  - A Colony Ship as soon as a Shipyard stands, flown every launch window.
  - Weight Moon, Mars and Venus, and hop Mars → Phobos/Deimos for the fourth Body.
  - Build a Refinery and a Trade Post at the first colony (T3).
- **S9. Computer Archivists research The Upload before over-filling the fund** [4/4].
  - Grow one ground colony to Colony tier while funding.
  - An Observatory over Earth before a Lab (T3), and Labs generally (T2).
  - Once the fund is full, Directive 0 and push The Upload.
- **S10. Only if the fixed AIs are still weak** [T2, T3]:
  - Arkwrights found with 3 Colonists (fits Coach Class).
  - Arkwrights start with an inland or walled Spaceport; T2 lost both Launch Sites to the sea on
    T19.
  - Earth L4/L5 counts as a Body.
  - Archivists may upload from any colony of theirs on the Archive's Body.
- **S11. Re-pace the human versions** [T1, T2, T4]:
  - Gate the Archivists' The Upload later (rung 5, or via S4).
  - Upload cap by Archive tier (T1), or 6 a turn to cut the dead end-turns (T2). The two pull
    in opposite directions; pick one.
  - Lengthen the Arkwrights' Capital step: e.g. 6 turns at Colony tier (T4).
- **S12. Cap or taper the Trade Post first, then re-measure both** [T2, T3]. Every human win in
  T2's games leaned on it.

### c. UI fixes and text reductions

- **S13. `show` prints pending offers with their terms** [4/4]: "Archivists offer: 20 Fuel for 30
  Energy (trade accept|refuse archivists)".
- **S14. Refuse lines, not turns** [T1, T2, T4]. Run the legal lines and list the refused ones,
  with `--strict` for today's behaviour. Check with look-ahead (T3).
- **S15. A Shortfall forecast in the order summary** [4/4]: "After them: 0F, Power Plants at a
  quarter" (see S2).
- **S16. One name per thing** [T1, T2, T4]. Accept the shown names ("reactor", "power plant",
  "exchange"), and drop other Factions' buildings from refusal lists (B8).
- **S17. Founding and crowding said at the order** [T2, T3, T4]:
  - "arrives with 3: needs 4 to found";
  - "founds with 4; 3 stay aboard";
  - "Colony Ship: 7 seats (+7 crowded, risky)".
- **S18. The Collapse line as a range or trend**: "Collapse T26–40, easing" [T1]. Or merge the two
  lines: "Collapse T41 (act by T34)" [T2].
- **S19. Rival ships bound for a Body you're racing to**: "A Custodians Colony Ship is bound for
  Mars, arriving in 2 turns" [T4, T1].
- **S20. Coastal flag at the order and in lists**: "Wind Farm (coastal: sea at +1.8 C)" and
  "(0 coastal free)" [T1, T2]. [hover covers, partly]
- **S21. Glossary leads with the current rule**, with the version history moved to an appendix
  [T3]. The 224 KB glossary was hard going; the Archive entry carries seven version clauses before
  the rule.
- **S22. Text cuts the testers proposed** (quote → proposal):
  - "Custodians win the race to the Moon. Prospectors had a Shipyard; Archivists had a Shipyard;
    Arkwrights had no Shipyard. Tycho on the Moon is theirs; Unrest eased by 0.5 in every
    Region." → "Custodians win the Moon race: Tycho is theirs. Unrest −0.5 everywhere." (T4; note
    that T4 also called the long form the best line in the game)
  - Victory line "Colonists beyond Earth orbit: 0/44 | 0 of 4 Bodies with 4 Colonists or more |
    score 0.00 (held: needs Generation Ships, not yet researched) | gate: Generation Ships not
    yet" → "Off Earth 0/44 | Bodies 0/4 | Gate: Generation Ships" (T2)
  - "Archive fund: 250 of a cap of 250; the Archive is complete at Mare Tranquillitatis on the
    Moon, where 33 Colonists stand to be uploaded; 0 remembered here" → "Archive 250/250,
    complete. 33 can upload, 4 a turn." (T2)
  - "next `upgrade iss`: Settlement (12 Modules) at 12 Colonists, 33 Materials and 4 Widgets" →
    "Next: Settlement (12 slots): 12 Colonists, all slots built, 33 M" (T2)
  - "would arrive stranded at: Mars (3.6 Fuel left, nothing of yours to refuel at in low orbit);
    Phobos (…)…" → "No fuel home from: Mars, Phobos, Deimos, Ceres" (T2)
  - "Overtime at the Yards is asking the Arkwrights a question, and it has not been answered.
    Answer it. The turn cannot end until you do." → "Answer the card: Overtime at the Yards."
    (T3)
  - "STILL OWED: Several things before the turn can end:" → "Owed:" (T3)
  - "*** THE OFFER IS CLOSED TO YOU: you have 34.2 Ducats of the 40 it asks, so `answer refuse`
    is your only move. ***" → "Closed: you have 34.2 of 40 Ducats. Only `answer refuse`." (T4)
  - "The turn did NOT end: 1 line was refused. Fix them, or pass --force to end the turn with the
    rest." → "Turn not ended: 1 line refused. Fix it, or --force." (T4)
  - The Rival report "Prospectors lifted 2 Pioneers from Canada to Tiangong over Earth, recruited
    2 Pioneers in Canada, spent 10 Influence on Russia, bought 5 Influence, spent 5 Influence on
    The United Kingdom and …" → grouped: "Prospectors: +4 Pioneers; 15 Influence (Russia 10, UK
    5); Exchange at Tiangong." (T3)
  - "REFUSED `build module mareimbrium mine`: full until more Colonists live here" → "REFUSED: no
    free slot (4 of 4; 2 land this turn)" (T3)
  - Break: "The ice sheets let go. Greenland and Thwaites pass the point of return; the sea is
    coming for the coasts and it will not stop." → "Ice sheets lost: coasts will flood." (T1)
  - help: "A LANDING OR FOUNDING IS SHUT ONLY BY A BLOCKADE OF LOW ORBIT, never by a Ship on Hold,
    never by a Passage partner." → "Only a low-orbit Blockade stops a landing." (T1)
  - help (refuel): "FREE OF THE SHIP'S TURN: give the refuel first and the transit after it, in
    one turn; it fills before it goes." → "Free; refuel then fly in one turn." (T1)
  - Ship card "colonists 8/6" → "8 aboard (2 crowded: may die)" (T1)
  - Colony "Modules 5/6" → "5 of 6 built, 1 free" (T1: the Core isn't counted)
  - Slot letters "[M 1.25 G 1.34 R 0.59 S 0.96 Fi 0.23]": G and S are explained nowhere the testers
    looked (T2). Spell them out once.

### d. Making the AI smarter

- **S1, S8 and S9 above**: denial of the leader, Arkwrights that launch, Archivists that research
  their gate.
- **S23. Contest the Moon** [4/4]. All computer seats built Shipyards by T1–T4 and none landed
  before the human's T5–T8. Send the first Colony Ship the turn it can fly.
- **S24. Use warships** [T1, T3, T4]. In 64 turns of logged AI play, one Frigate was built; it
  parked or shuttled and never fought. Blockade the leader's ferry colony.
- **S25. Defend home, and spend Influence sensibly** [T1, T2, T4].
  - Answer when a home Region is being bought: the Prospectors lost Europe to one turn of
    Influence and replied with 10–15 a turn.
  - Don't pour Influence into a Region held at 263 (T2).
  - Don't mirror a human point for point on a 1-slot Region: ~100 Influence wasted each (T4).
  - Contest neutral climbs earlier: ten Regions fell uncontested (T3).
- **S26. Vary the post-Accord attack** [T1]. The Custodians took Mexico on T11–T13 in 2 of 2
  games when the Accord lapsed: good drama, but predictable.

### e. Bolstering colonization

- **S27. Ferry routes worth trusting** [T1, T3, T4].
  - Show what a route loads from, refuel it automatically at each end, skip a leg when the
    destination is full.
  - Report "Magellan waited: Tycho full" or "Magellan idle, 6 aboard".
  - T4 never set one up, and T3 never needed one on a one-turn Moon hop.
- **S28. Each Body a reason to go** [4/4]. The Moon alone did everything.
  - Unique yields at distance, e.g. Fissionables that matter for nuclear hulls and Reactors.
    T4 paid 8 Ducats per Fissionable while Vesta sat empty.
  - Mars Observatories at ×2.
  - Phobos as a cheap depot.
  - Exchange pay shown on the FREE SLOTS list.
- **S29. A new Colony opens with the whole load, crowded**, instead of seating 4 and stranding the
  rest [T4, T3]. Or 3 Colonists found an Outpost [T1, T2] (overlaps S10).
- **S30. Off-world self-sufficiency** [T4, T2].
  - Lunar Generators burning Earth's Fuel broke immersion, and the Moon went dark (T4 G4).
  - Colony power from local yields.
  - Fuel as a chronic bottleneck: a Refinery makes 4, a Ship wants 32. Reserve a ferry Fuel pool,
    or let a station turn Energy into Fuel.
- **S31. Faster first-colony building** [T3]. The Core's 4 Widgets a turn makes every build crawl
  unless Ducats are paid (E2): a Core at 6, or a free first Factory.
- **S32. Colony events and growth that shows** [T4, T3]:
  - colony Choice Cards (a dust storm at Valles, a dome breach, a birth boom);
  - per-colony milestones in the Chronicle;
  - growth above 2% a turn, or a Module that lifts it.

### f. Conflict, war and spaceships

- **S33. War has a reason before T20** [4/4]. Testers saw zero ship battles in 16 games. All
  fighting was Armies on Earth, nearly all of it against the human after a lapsed Accord. A
  Blockade that halves a rival's Trade Post or Exchange income gives every Faction a reason to
  build a warship. It also counters E1 (T1, T2).
- **S34. Warships that move a Victory bar** [T3, T4, T1]:
  - a Frigate ambushes a Colony Ship at its arrival orbit in a predictable Mars window;
  - a Battleship bombards an Archive or Capital under construction, for a one-turn delay;
  - a Carrier seizes an Exchange colony;
  - escort as one order: `escort <warship> <colony ship>`.
- **S35. Threats said aloud** [T1, T2, T3]:
  - "PMV Valiant (Frigate, Attack) is in low orbit over your Moon colony";
  - "Custodians Army (str 4) borders India";
  - raise an Army there in one order.

  T3: the CLI's colony row says only "(threatened)".
- **S36. Accords as a declared state** [T2]. Non-aggression between Friendly seats renews itself;
  a lapse reads "Custodians may attack".
- **S37. Missile Technology as a moment** [T1, T2]. Researched in three games, by accident in one,
  and nobody fired a missile. A confirmation line, a Relations hit or a card.
- **S38. Ships with histories** [T4]. "Veteran of 6 Moon runs, 48 Colonists carried" on the card;
  ships in the Chronicle.
- **S39. Nuclear hulls earlier** [T4]. Nuclear Rockets came on T24–25, after the frontier was
  settled.
- **S40. Armies ride Colony Ships, or a cheap early Carrier** [T1]. Mexico couldn't be answered
  from China: "not a neighbouring continent".

### g. Each tester's extra dimension

- **S41. Onboarding: a goal ladder for the first 10 turns** [T1, T2]. Extend the Signature Moves
  line, "the most useful text in four games" (T1) and "the best text in the game" (T2), into
  five steps per Faction, each with a tick and a small Moment. Arkwrights, for example:
  - Station;
  - Shipyard;
  - Colony Ship with 4+ aboard;
  - Sea Wall your Spaceport;
  - Refinery before your second Ship.

  T1: "I'd have quit at turn 3 without the signature-moves line."
- **S42. AI depth: each AI reads its rivals' Victory bars** [T3]. This is S1, chosen as the
  dimension that lifts the game from 6 to 7.5–8.
- **S43. The people of the colonies** [T4]:
  - a colony Morale or Unrest driven by crowding, Energy shortfalls and births;
  - second-generation Colonists counted apart, worth more off Earth;
  - late Independence pressure on big colonies.

  "The first-child Moment was the one time the people felt real, and it was the best moment in
  all four games."

## 6. Ratings

| Tester | Rating | What would raise it, in their words |
|---|---|---|
| 1, common gamer | 6.5 | "I'd have quit at turn 3 without the signature-moves line… once a number was climbing it hooked me hard." Loses points because rivals never fight for your win and research empties the tree by mid-game. |
| 2, casual 4X | 6.5 | "Fix the warnings and the Trade Post and this is an 8." |
| 3, hardcore | 6 | "With an AI that plays for denial, this is a 7.5–8." |
| 4, colonization | 6 | "Give space a second act, make the AI launch, and let Earth's collapse be a chapter rather than the end, and this becomes an 8." |
