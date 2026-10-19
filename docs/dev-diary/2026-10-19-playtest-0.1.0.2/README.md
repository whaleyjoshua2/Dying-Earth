# Playtest of version 0.1.0.2: four personas, sixteen games

Four independent agent players, one persona each, played the first 30 turns of every Faction (four
games apiece, seeds 501–504, 601–604, 701–704, 801–804) through the text tool `play`, against
three computer Factions. Each had only the game, the player README, the glossary and, from
mid-run, `mouseovers.md` (the window's hover texts, extracted from the interface so a text-tool
player could see what a window player is told). None saw the source, the docs, the tracker or the
0.1.0.0 playtest. Their turn-by-turn logs and final reports are `tester-1-notes.md` to
`tester-4-notes.md`.

| Tester | Persona | Custodians | Prospectors | Arkwrights | Archivists | Rating |
|---|---|---|---|---|---|---|
| 1 | Common gamer | no win, 51% at T30 | **won T26** | **won T30** | **won T21** | **6/10** |
| 2 | Casual 4X | **won T28** | **won T31** | 100% held at T30 | Archive full, Upload not yet picked at T30 | **6/10** |
| 3 | Hardcore grand strategy | **won T22** | **won T23** | **won T25** | **won T18** | **5/10** |
| 4 | Colonisation fantasy | no win, 50% from T14 | lost T26 (AI Archivists) | 100% held at T30 | **won T27** | **6/10** |

**Eleven wins in sixteen games, all before turn 31 of 50, against a computer that won once.** Every
tester won the Moon race in every game (16 of 16); nobody saw a computer seat reach Mars first.
The 0.1.0.0 playtest had the same four ratings (6, 6, 6, 6); the hardcore player has dropped to 5
because "every Faction has one loop that wins by turn 18–25 against an AI that never stops it."

A caution on reading this: the players used the **text tool**, not the window. Each finding below
is marked **[window: covered]** where an existing mouse-over or panel already answers it, so a tool
artefact is not mistaken for a game gap.

## 1. What grabbed them

All four, independently, named the same two things:

- **The race to the Moon** (turns 6–10, every game). "THIS is why I'm here. The race framing is
  perfect" (T4). "A real Civ-wonder moment" (T2). The line naming who was close behind ("Arkwrights
  had Colonists aboard; Custodians had no Shipyard") drew praise twice.
- **Taking a country by Influence**, the first time (turn 3). "Pick a country, pour points, it
  flips. Simple and it works" (T1).

Then, less often: a ship with a name ("TSV Magellan arrived at the Moon. MY SHIP HAS A NAME", T4);
first on Mars and the Venus cloud station; the Break texts; a 10% battle won (T1); and the one time
the computer fought back over a capital (T2's Prospectors game, "the most alive the game has
felt"; T3's Arkwrights game, four Armies on their capital the turn their Accords lapsed).

## 2. What lost them

- **The Custodians' climate half reads as impossible** (T1, T2, T4), and the answer is a building
  nobody mentions. T2 found Scrubbers on turn 21 by typing a junk facility name; T1 never found
  them and finished at 51%; T4 sat at 50% for sixteen turns. T3 found them and called the Faction
  "Scrubber spam".
- **Victory stalls on a gate Tech the player can't steer toward** (T1, T2, T4; six games). The
  Extraction Charter / Generation Ships / The Upload never appear on the Research Lead's shortlist
  of three, nothing names the prerequisite, and the top line reads **"Your Victory: 100%"** while
  the win is held. "I thought I'd won" (T2). Three testers were offered a *single* Tech, a rival's
  gate, as the "free choice". Note: the Tech Tree window shows prerequisites and the Victory window
  prints "needs X" on the part's line; the 100% headline does not. **[window: partly covered]**
- **Logistics bury the frontier** (T2, T4). Change-orbit, refuel, load, fly, unload: four or five
  turns a shipload. **The 15.6 / 15.7 Fuel trap** hit T4 in three games and T2 once: a 35 tank less
  Earth→Moon (15.7) less Moon→Earth (3.7) leaves 15.6, a tenth short of a second trip.
- **Nothing to spend on.** Energy at 200–450 and Fuel at 150+ with no sink (T2, every game; T4);
  Materials at 85–170 a turn (T1), which is what fed the Influence money printer.
- **Turns become a conveyor belt** from about turn 13 (T2, T3): "no decisions after T8, just how
  many Scrubbers this turn."

## 3. Exploits, in order of how many found them

1. **Influence buys the world** (4 of 4). Neutral Regions cost 22–41 against an Allotment of 16–30
   and the computer almost never contests the one you pick; its own turn-2 non-aggression offers
   then make your holdings immune. T3 held 13 Regions in game 1 without moving an Army. **Sold
   Materials and Fuel buy Influence at 4 Ducats a point** (T1, T2): T1 took China and Europe in one
   turn and **all twenty Regions by turn 23 as the Prospectors**. A home Region flips as easily as
   any other: T2 took the Prospectors' China (and later the Custodians') in one turn for 80 Ducats,
   Investment Banks and both Armies included; "no war, no warning, no defence."
2. **Earth-orbit stations count as "off Earth"** (4 of 4). A 44-Material station and two lifts a
   turn meet the Custodians' and Prospectors' 18 by turn 9–14 without a ship leaving orbit; one
   `lift southafrica 30 24` put 30 Pioneers on Orbital Reef for +30 Influence and 64/42 (T4). "Most
   off-world Colonists in my games were 400 km up."
3. **Scrubbers** (T2, T3): 33 Materials, +~3 ppm Sink each, done next turn, cap-free; "10x every
   other climate lever". Card and permafrost emissions don't count toward Stabilization (T3), so a
   Custodian refuses every climate card free.
4. **Investment Bank stacking** (T3; T2 by use): one per slot, +11.5 Ducats for 23 Materials,
   payback five turns, doubled by Strip Permit; income 16 → 300 a turn by T12; the computer never
   attacked a Bank.
5. **Colonies off Earth bought by Influence from home** (T1): Tiangong, Stickney, Swift, and
   colonies on Mercury, Vesta and the Moon, no ship, no travel. "Why fly anywhere?"
6. **Research Lead denial** (T3): whoever leads picks from three, so a human never picks a rival's
   gate. Conversely the human is sometimes forced to (one-Tech shortlists, T1, T4).
7. **A completed Occupation flips straight back** to whoever holds more Standing in the same
   Resolution (T2, T3), so "war only wins where nobody has Standing."
8. **Cheap Ore refused** sets Materials at 0.9 for everyone with no cap on buying (T3: 220 in one
   order). **Distress Call** pays +5 Relations for idling a ship (T2). **The Upload** ends the game
   in one order once 18 live at the Archive (T1, T4).

## 4. Bugs and suspected bugs

- **"ok" orders that did nothing**: three orders answered ok and the ship arrived at the Moon empty,
  no Report line (T1, T9). `unload` ×2 said ok and 8 Colonists stayed in orbit after the first 4
  founded (T4).
- **An accepted Influence order breaks an Accord** (all four, seven times): a Region a partner took
  the same turn, or Influence on a partner's land in the same order list as the acceptance. The
  text tool says "ok … breaks your Accord"; the window's Influence button says "breaks your Accord
  with X" beside its price (ticket #520). **[window: covered at order time]**, but a standing Max
  order or a same-turn handover still slips through.
- **Orbital Control ignores a Passage Accord**: a partner's Frigate on Hold blocked a landing at the
  player's own colony with non-aggression and passage standing (T4, two games).
- **Techs "led by" the player that the player never picked**, opening all three rivals' gates under
  their name (T1, games 3 and 4; possibly two Techs completing in one turn).
- **The Venture Fund grows faster than "banked last turn"** (T1, T2, T4): a Fund of 7,314 of 3,500
  (T2); +566 in a turn that reported 90. *Not a bug:* each Investment Bank banks 1% interest a turn
  on the balance, one per Region, and objectives pay in. The window's income list names the
  interest; the text tool's "banked" line does not, and the compounding is the Prospectors' late
  runaway.
- **Display**: a station reading "Modules 5/0" with 0 Colonists; the Custodians at Blame 0.00;
  a coast-loss headline saying +1.8 C against a Climate line of +1.28 (T4). The Report header
  repeats last turn's card question after it was answered (T3; tool).
- **Tool**: `influence tiangong` refused because the error lists only Regions; `lift … 0` answers
  "6 Pioneers are waiting there"; `transit … earth low` says "already there" when `change-orbit`
  was meant; the order list runs top to bottom so `sell` must precede what it pays for; `-ducats`
  help says "twice its Materials" but charges Ducats; "needs 4 Pioneers here" doesn't say where.

## 5. Suggestions, synthesised, by the dimensions asked

Each line gives who raised it and, where the window already answers it, says so.

### Make it more interesting and compelling
- **Pressure between the win conditions** (T3's extra dimension; T2, T1 by implication). Every game
  was four solitaire races: nothing a rival does slows your bar. One lever per Faction against
  others' bars: Custodians sanction a high-Blame Faction's Banks; Prospectors buy out a rival
  colony; Arkwrights poach Pioneers; Archivists embargo Research toward the Lead. The computer
  already judges who leads and could use them.
- **Name the signature moves on turn 1** (T2): Custodians "Scrubbers, Nature Reserves, Leapfrog";
  Prospectors "Investment Banks, Venture share"; Arkwrights "Colony Ships to four Bodies";
  Archivists "Labs, Directive, Archive on the Moon".
- **A mid-game rival crisis** around turns 10 and 20 (T1, T2): a card or Accord that lets the table
  gang up on whoever is at 75%, which the Report already announces.
- **Colonies with names and a story** (T4's extra dimension): `name 27 Shackleton Base`; a
  one-line flavour status per tier; milestones; an ending that recites each colony, its people and
  who was first. "Every high point was a story beat; every low point a counter that wouldn't move."
- **A mid-game cost to each win** (T3): the Archive draws Energy per Colonist waiting; the Fund
  loses a share when a Break fires.

### Balance
- **Influence**: raise neutral thresholds by +5 per Region already held (T3); a home Region at ×2
  with the computer defending it when a rival passes 75% of the threshold (T2); each `buy-influence`
  within a turn dearer, or large sales crash the price (T1); Influence on colonies off Earth wants
  presence, a ship in that orbit or a Relay, or a threshold scaled by distance (T1).
- **Off Earth means off Earth** (4 of 4): count only Colonists beyond Earth orbit, or Earth-orbit
  stations at half, or require one Colony beyond Earth orbit.
- **Scrubbers**: cap per Faction (~8) or per Region (1), raise their Energy upkeep, or diminishing
  returns (T2, T3); count card and permafrost emissions toward Stabilization at half (T3).
- **Investment Bank**: one per Region, or each extra pays half the one before; exclude Banks from
  Strip Permit (T3). Cap market buys per turn or raise the price per unit bought (T3).
- **Gates**: rung-4 gates need two rung-3 Techs; force a Faction's own gate onto the shortlist once
  its prerequisites stand, and never offer a one-Tech shortlist that is a rival's gate (T1–T4).
- **Archivists**: cap uploads (~4 a turn) or make the Archive run powered for K turns first (T1,
  T4). **Arkwrights**: a Research floor toward Generation Ships, starting Ducats above 1, a Solar
  Array on Orbital Reef from the start (T4); Arkwrights and Archivists make 1–6 Ducats against the
  Prospectors' 80–170 (T2).
- **The computer Prospectors run away** and cook the planet (T2, two games; T4): a brake on their
  Strip Permits and conquest pace, and a Prospector that weighs Collapse.

### UI fixes, adjustments and text reductions
- **"Your Victory: 100%" never while held**: "100% — needs Generation Ships" (T1, T2, T4). The
  Victory window's part line already says "needs X"; the headline percentage does not.
  **[window: partly covered]**
- **Show the gate chain** in the Victory block from turn 1: "gate: The Extraction Charter (needs
  Rail Gun Freight → Automated Refining)" (T1, T2, T3). **[window: Tech Tree shows prerequisites]**
- **Refuse, or demand an explicit word for, an order that breaks an Accord** (T1, T2, T3, T4).
  **[window: the price line says "breaks your Accord with X" (#520)]** Still wanted: a refusal, and
  a warning when an accept and a breach stand in the same order list.
- **Echo the building's name on index orders**, and refuse if the index now points elsewhere (T3,
  who scrapped their own Launch Site after the sea shifted the numbering).
- **A Report line when the sea takes one of your buildings** (T1, T3); fold the twenty "The sea
  took 1 coastal slot" lines into one per player; an invasion of your Region as the headline (T1).
- **Count Colonists aboard a ship** as off Earth or label them "in transit" (T1, T2).
  **[window: covered — "(+N in transit)" and the hatched band on the bar]**
- **Battle odds before the order** (T1, T2). **[window: covered — the attack button's hover]**
- **Costs before the order** (T1, T2, T3). **[window: covered — build buttons carry the cost]**
- **"Last turn to act" flip-flops** between a number and "the world is safe" (T1, T2).
  **[window: the hover explains it]** Still wanted: one stable figure and a trend.
- **The Venture Fund's "banked" line** should name the interest (T1, T2, T4). **[window: the income
  list names "N Investment Banks (interest banked)"]**
- **Orbits by station name** (`change-orbit 27 tiangong`), since colony ids and slot numbers differ
  for the same station (T2, T4; tool, and worth a look in the window's Ship card).
- Text cuts in the window's hovers, each proposed with a one-line replacement (T2, T3, T4): the
  Max-repeat hover; the Allotment hover's eight sources ("Influence a turn: {n}. Unspent is
  lost."); Unrest's four thresholds ("Unrest {}: Army stops at {}, Facilities halve at {}, revolt
  at {}. Falls {} a turn."); the Research Directive's Lead note; the overcrowded Habitat; the
  Accord-lapse note; the first-landing prize; the Pioneers-waiting line; the Blockade and Upload
  texts.
- Text cuts in the tool: "calm enough: the Standing Army replenishes …" on every Region (print
  only when not calm); "Standings: nobody has any yet"; the Standing clause on every Region line;
  ticket and version references in `help`; the tied-Lead sentence. Tool additions: a build list
  with costs; `turn --check`; owed answers at the top of `show`; a Tech tree view; a "refuse all"
  order; neighbours per Region.

### Make the AI smarter
- **Defend the home Region** with Influence when a rival nears the threshold (4 of 4); defend any
  Region past half-threshold (T3).
- **Build a Shipyard and Colony Ship by turn 4 and take the first Mars window** (4 of 4: the human
  was first to the Moon 16 of 16 and to Mars whenever tried).
- **Use the Research Lead to deny the leader's gate** (T3); **gang up earlier**, at 60–70%, and on
  Influence runaways too (T1); contest a rival one step from winning (T4: 16/18 uploaded, nobody
  moved).
- **Stop re-offering** an Accord refused last turn or already standing, and the same rejected
  trade; renew in the last turn instead of letting it lapse (T1, T2, T4). Re-sign or arm before its
  own Accords lapse (T3).
- **Follow a conquest with Influence** so the Region doesn't flip back; attack undefended rich
  Regions (T3).

### Accessibility in the first few turns
- **A five-line turn-1 card per Faction**: the win in plain words, the first three orders, the one
  building that matters (T1, T2, T4). **[window: partly covered — the tutorial notes at the head of
  the first turns; the testers saw none in the tool]**
- **The gate chain and its Research total** on screen from turn 1 (T1, T2, T3).
- **Costs and what can be built** on each place's card (T1, T2, T3). **[window: covered]**
- **The Mars window as a countdown** from turn 1; each ship's round-trip Fuel need; "a Colony with a
  Refinery refuels ships in its low orbit" said at the start (T4).
- **A Stabilization panel for the Custodians**: the world's emissions by source, the Sink, their
  three biggest levers in ppm (T1). **[window: the Climate Panel splits emissions by source; the
  levers are not ranked]**

### Bolster the colonisation side
- **Auto-refuel at a held Launch Site** in Earth's low orbit, from the Stockpile (T4): kills the
  15.6/15.7 trap and two turns of every trip.
- **A standing ferry route** (`route <ship> <from> <to>`) that loads, flies, unloads and returns
  until cancelled (T1, T4).
- **Board and go** in one turn at a yard; **the whole crew lands** on founding, the rest crowded
  rather than left in orbit (T2, T4).
- **Refugee Convoy offers "send them to the stars"** (T4): the refugees become Pioneers at a Launch
  Site or land at your largest colony. "The game's theme, and the card can't do it."
- **Bodies that differ**: one unique module or resource per Body (Mars Research, Mercury Energy,
  Vesta Materials, Venus clouds, Ceres ice); dust storms, heat, Energy imports (T1, T2, T3, T4).
  Today "the site yields never changed a decision of mine" (T1).
- **The Moon contestable** and more Moon sites (T3, T4: all four taken by T15, then nothing nearby).
- **Visible growth per tier**: Settlement gets a name and flavour; Colony a local Shipyard
  discount; Capital raises the Body's flag with a Moment; colonies make more Widgets as they grow
  (the Settlement-to-Colony step on Mars is ~9 turns at 4 Widgets a turn) (T4).
- **Colony events** (births, a dust storm, a first harvest, independence) beside Region events (T4);
  colonies grow their own Pioneers; a stranded ship scraps into the colony below (T1).
- **Make colonies matter to every Faction's win** (T4, T3): for the Custodians, Colonists off Earth
  lower per-person emissions or a colony Scrubber module.

### Improve conflict and war mechanics
- **Occupation sticks**: pause Influence on a place while an Army occupies it, or raise the
  occupier's Standing to the old owner's plus the margin (T2, T3).
- **Orbital Control respects Passage**; say "X's Frigate holds the orbit; offer passage or bring a
  warship" at the order (T4).
- **Space war reachable**: no Ship attacked a Ship in sixteen games. Let warships intercept Colony
  Ships in transit (T3); a threats line on the colony card and a note when a rival warship arrives
  at your Body (T4); a Battery the colonist player can afford.
- **Armies travel by ship**, across oceans and to colonies (T1: "not a neighbouring continent" was
  the first word of adjacency); show neighbours; a "retreat under 30%" option; Barracks reinforce
  automatically (T2, T3).
- **A cheap "liberation" counter** for the former owner of an Influence takeover (T1).

### Each tester's own dimension
- **T1, cause and effect**: an end-of-turn "what changed for you" box, five lines with causes
  ("Fund +312: banked 100, interest 212"; "Japan threshold 32 → 35: rival spent 5, your Standing
  decayed 2"). It would also surface the suspected bugs.
- **T2, economy sinks**: Energy sellable or convertible to Materials; Fuel with a use beyond ships;
  Scrubbers that eat Energy; a "your bottleneck: Materials" hint.
- **T3, pressure between the win conditions** (above).
- **T4, colony story and identity** (above).

## 6. What the window already answers

For the designer's eye, the complaints that are text-tool artefacts rather than game gaps: build
costs before the order; battle odds before the attack; Colonists aboard a ship shown as "+N in
transit"; the "Last turn to act" explanation; the Accord-break warning at order time; the Venture
Fund's interest in the income list; the Research Directive's cost to the Lead; the Tech Tree's
prerequisites; the first-turn tutorial notes; the emissions split by source. Everything in sections
3 and 5 that is not so marked is a gap in the game itself.
