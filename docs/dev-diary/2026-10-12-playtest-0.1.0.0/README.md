# Playtest of version 0.1.0.0: four players, sixteen games

Four independent agent players, one persona each, played the first 25 turns of each Faction (four
games apiece, seeds 101–104, 201–204, 301–304, 401–404) through the text tool `play`, against three
computer Factions. Each had only the game, its assets and the player README; none saw the source,
the docs or the 9/24 playtest. Their full turn-by-turn logs are `tester-1-notes.md` to
`tester-4-notes.md` beside this file.

| Tester | Persona | Custodians | Prospectors | Arkwrights | Archivists | Rating |
|---|---|---|---|---|---|---|
| 1 | Common gamer | **won T26** | no win (gate never offered) | no win (no Capital order) | **won T22** | **6/10** |
| 2 | Casual 4X | lost the thread T24 | on course ~T30 | 3 of 4 Bodies (Venus) | 16/18 uploaded, Archive dark T25 | **6/10** |
| 3 | Hardcore 4X | no path | **won T20** | 4/4 Bodies, gate never offered | **won T23** | **6/10** |
| 4 | Colonisation fantasy | out of reach | 1,966/3,500, racing collapse | 43/42 on 5 Bodies, gate held | Archive full T18, gate held | **6/10** |

**All four rated it 6/10, and all four said "8 if fixed".** Every one named the Arkwrights the most
fun Faction, and taking countries by Influence the best early loop.

A caution on reading this: the players used the **text tool**, not the game window. Findings that
belong to the tool are kept apart below.

## 1. Bugs (game)

1. **Venus soft-lock — seen in five games by all four testers.** A Colony Ship in a Venus station
   orbit (Ishtar) cannot found anything. `unload … into slot venus N` says "move to low orbit";
   `change-orbit <ship> low` is refused; `transit … venus low` says "already there"; `build station
   venus N` says "a Colony Ship in this orbit founds it". `show` lists no low orbit at Venus. Ships
   stranded with 4–9 Colonists. Repro: tester 4, seed 401, ship 41, turn 22; seed 402, ship 44.
   **Fixed (2026-10-12).** The engine always could: a Colony Ship founds a station from the ring it
   sits in (ticket #442), and the window offers it as "Found …" on the Ship card. The text tool had no
   way to say it, and the refusals pointed at a low orbit Venus does not have. The tool now takes
   `unload <ship> <n> into ring <body> <slot>`; a slot founding at Venus says "Venus has no ground to
   found a Colony on: a Colony Ship founds a Space Station from the station slot's ring it stands in";
   a change to low orbit says "Venus has no low orbit, only 3 Orbital Slots". Tested (watched red on
   the old message), and replayed on tester 2's stranded save: Ishtar founded with 4, 5 aboard.
2. **Accord accepted but not struck** (testers 2, 3). `accord accept X` echoes "you accept", yet no
   Accord stands and X offers again; in tester 2's case X marched Armies in the same Resolution. Both
   cases were renewal-type offers — suspect the legacy Renew path left in place by ticket #509.
   **Fixed (2026-10-12, ticket #517).** Not the Renew path: the Accords were struck, then broken the
   same turn by the computer partner spending Influence on the player's Regions (non-aggression is
   "no Influence or Battle on each other"), and the break went only to the log. The computer now
   drops any order that would cross a non-aggression partner (Influence, Agitate, Smear, an Army
   into, a Bombard or Launch at its places), and a broken Accord is told to each player in it. Swept
   (`../2026-10-07-version-0.1.0.0/sweeps/ticket-517.txt`): **0 / 30 / 13 / 18, 19 collapses**, from
   7 / 27 / 17 / 7 and 22 -- the Archivists up past noise, the Custodians to nought.
3. **Phantom offers** (tester 2). "STILL OWED" lists a Trade from the Custodians and the Archivists,
   but `trade accept|refuse` answers "they have offered you no Trade"; and Accord offers for an Accord
   struck earlier the same turn block the turn (accept: "you already hold an Accord").
   **Fixed (2026-10-12, ticket #518).** Three causes. Striking an Accord swept every other offer
   between the pair, Trades included, so a Trade the turn owed was gone when answered: now only
   Accord offers are swept. A new Accord offer to a pair that already holds one is dropped at the head
   of the turn. And in the text tool a yes that could not go through (a Trade whose goods are gone, an
   Accord already standing) counted as a refused line, so the run was not saved and the turn owed the
   offer again: it now counts as answered, saying why.
4. **Inherited Armies start wars** (testers 3, 4). A Region taken by Influence hands over the old
   holder's Armies still on Attack; they fight unordered, bring Hostility, and in seed 404 conquered
   China and destroyed a Sea Wall.
5. **Agitate did nothing** (tester 3): 15 Ducats + 5 Influence for "Unrest from 0 to 0", accepted
   under a non-aggression Accord with the holder.
6. **Carbon Levy** (tester 3): the card says +0.5 Unrest; the Report showed Unrest falling. New card.
7. **Influence bonuses not visible** (tester 4): the Spaceport's "+N Influence next turn" and first-to-
   settle bonuses never showed in "Influence allotment this turn".
8. **Contradictory ownership headlines** (testers 1, 2): "X now belongs to the Prospectors
   (Occupation complete)" and "X now belongs to the Custodians (Influence)" in one Report; every
   ownership change prints twice.
9. **A win announced late and quietly** (tester 1): "score 1.00 MET" with "the game is still on"; the
   win came a turn later as a buried [Note] while the headline was another country.
10. **Tech id mismatch** (all four): "Large Language Models" can only be ordered as `publicscience`.

## 2. Design and balance (the big ones)

1. **The gate Tech depends on holding the Research Lead.** Testers 1, 2, 3 and 4 each met every bar
   in some Faction and sat "held" because their gate never came up on a shortlist they could pick
   from (Extraction Charter, Generation Ships, The Upload). The Archivists escape it because Labs feed
   both the Archive and the Research Lead.
2. **Influence can be bought without limit, at 2 Ducats a point.** Every tester found it: Ducats buy
   any country in a turn, rival capitals included. It is what makes the Prospectors (human and
   computer) snowball, and why occupation means nothing against a rich player.
3. **The Prospectors are overtuned.** Uncapped Investment Banks (whose interest also compounds the
   Fund), free Strip Permits that double a Region, and bought Influence. Tester 3 won on turn 20
   "never threatened"; the computer Prospectors ran away in most Custodian and Arkwright games.
4. **The Custodians' win feels out of their hands.** It needs the whole world under the Sink, and no
   order cuts a rival's emissions. Tester 1 won anyway with a wall of Scrubbers (India took 10) and
   mothballing; testers 2–4 found it unwinnable once the Prospectors grew.
5. **Off-world Mines break the economy.** About 35 Materials a turn each against 9 for an Earth Mine;
   190–376 a turn by turns 16–20, sold for Influence. "Money stops mattering around turn 15" (tester 4);
   two testers sat on 180–300 idle Materials late.
6. **Home Regions lost without warning.** Standing at home decays about 1 a turn; three testers lost
   their home or only Launch Site to a rival's Influence with no alert.
7. **Accords lapse silently** at five turns; a rival then attacks or takes a country the same turn.
8. **The off-Earth bar without a Ship.** Custodians and Prospectors meet "18 off Earth" by stacking
   Habitats on the starting ISS and lifting every turn.
9. **One order per Ship per turn** makes load + fly + land three turns, and missed a Mars window for
   tester 2 (off-window Mars costs 40 Fuel against a 35 tank).
10. **The Arkwrights' start can be very slow** (Brazil: 1 Ducat a turn, five turns to a Colony Ship),
    the most colonisation-themed Faction starting slowest.
11. **The 20-Energy Archive** went dark the turn tester 2 was about to win.
12. **The computer:** sharp at Influence sniping, weak elsewhere — it mothballs its own economy,
    builds and scraps Schools, sends Armies one at a time, never attacks Colonies in space, never
    gangs up on a runaway leader beyond smearing.

## 3. Information the player needs and doesn't get

- **Offer contents:** Trades and Accord offers must be answered without seeing their Terms (all four).
- **Rivals' Standing** in a country; a warning when a rival is spending on one of yours, when your
  home is at risk, and when an Accord ends next turn.
- **Costs, yields and what a building does** (in the window these are on the build buttons; in the
  text tool nowhere). What Leapfrog does. Each building's emissions before you build it.
- **The founding rule** (four people) before launch; what "needs 4 Colonists" means for a station
  (4 Pioneers waiting in a Region with a Launch Site); a warning when Pioneers are recruited where
  there is no Launch Site.
- **The Influence order's result**: "your Standing would be X of Y needed".
- **The Tech tree's prerequisites**, and the route to each Faction's gate.
- **A launch window opening next turn**; turns of Energy left; your own losses (sea level, occupation,
  shutdowns) under "Your works".
- **The win as the headline**, and the doom clock and Victory progress at the top of every Report.

## 4. Text-tool gaps (not the game window)

- **No order to raise a tier** (Settlement, Colony, Colonial Capital). All four hit it; it made the
  Arkwrights' Capital requirement **untestable** in this playtest.
  **Fixed (2026-10-12, ticket #526):** `upgrade <colony>` raises a Colony or station to its next
  tier, and `show` prints each place's tier and what the next costs. Checked on tester 1's
  Arkwrights save: South Pole-Aitken (25 people) takes `upgrade 27` for 30 Materials and 4 Widgets.
- No view of an offer's terms; no `--check` dry run; `help` doesn't list Region ids or costs and
  carries developer ticket notes; Region display names ("usa", "the united states") not accepted in
  orders (`united` resolves to the UK); ships addressed by slot number, not station name; Energy can't
  be sold.

## 5. What delighted them

1. **"First to settle X"** races and landings — the Arkwrights were everyone's favourite Faction
   ("peak game moment of the whole playtest": first on Deimos and Mercury in one turn).
2. **Taking countries by Influence**, like Civ city-states, with a clear "26 of 30 needed" counter.
3. **The climate as a shared clock**: the sea-level Break wiping coasts worldwide, Coral Die-off,
   Permafrost Thaw, the doom clock "the world collapses on turn 49", racing your own forecast.
4. **Flavour**: real place names (Mare Tranquillitatis, Stickney, Caloris), named Ships, Lake Vostok
   under the ice, "the first Colony on the Moon eased Unrest in every Region".
5. **A refused order never ends the turn, and valid ones echo their cost** — a safe build menu.
6. The Energy alarm naming what goes dark next; progress numbers that move every turn; the Research
   Lead tug of war; launch windows and Fuel arithmetic; mothballing as a real lever.
