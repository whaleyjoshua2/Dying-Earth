# Dying Earth — version 0.08.5, the war version: the computer seats may attack anyone, Battles pollute and are reported by name, neutral Regions arm, a Blockade starves, a Greenwash for one's own Blame, the sea reaching inland, and the sweep counting the war

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.08.5](https://github.com/whaleyjoshua2/Dying-Earth/issues/275), and the
pictures and measurements that decided it are in
[`docs/dev-diary/2026-09-20-version-0.08.5/`](../dev-diary/2026-09-20-version-0.08.5/).

**What the version is.** Version 0.08.4 was about what the seats can do to each other with
Influence and Ducats, and ended with the Prospectors winning **28 games of 80** and the Archivists
none. This version is about the war, which four reviews filed the night before charting
([`docs/dev-diary/2026-09-19-combat-and-military-review/`](../dev-diary/2026-09-19-combat-and-military-review/))
had found to be silent, clean and one-sided: no Battle was a line of the Report, no Battle put
anything in the air, a neutral Region had no agency, a Blockade shut a slot "and nothing else", and
a computer seat could march only on a Region it had lost. Five tickets arm the table (§4, §6, §7,
§9, §3), one makes the war legible (§6), one makes it dirty (§4), and one teaches the sweep to count
it (§11), which no version before this could. The rest is the designer's housekeeping asked for in
the same breath: the sea reaching inland past any wall (§1), a Greenwash as the Smear's mirror (§2),
buildings that say what they do (§5), yields in glyphs on the planet card (§8), and the carbon
credits moved to the Faction window (§10).

**What it did to the win column**, 20 seeds across four seatings at the shipped climate cell
(sink 6, step 300), against the 0.08.4 baseline:

| Faction | 0.08.4 | 0.08.5 |
|---|---|---|
| Custodians | 26 | **28** |
| Prospectors | **28** | **26** |
| Arkwrights | 7 | **9** |
| Archivists | 0 | **2** |

**The column moved, a little, and away from the Prospectors.** The war is a Ducat sink for the
seat with the Ducats: where the Prospectors go second they build 46 Armies and 72 warships over the
batch and make a median 1053 Ducats a game where they made 1575; where they go first the Custodians
now win 5 of 20 where they won none. The Archivists have two wins, their first since 0.08.3, both in
the seating where the Prospectors' Ducats went to war. Five rules pressed on one seat and the sum
took two games of eighty off it, which is inside the noise of a reshuffled deck and is also the
first version since 0.08.2 in which the column moved *toward* the seats at the bottom. The designer,
shown the column: *"q1 ship"*.

Collapses fell from **19 of 80 to 15 of 80**, and every seating ends cooler (+2.87, +2.96, +2.89 and
+2.19 against +2.98, +2.96, +2.93 and +2.25). A falling collapse rate is not by itself good news:
the Collapse Line is the game's clock. The likely cause, measured but not proven: the sea reaching
inland (§1) takes more slots a game (27 to 36 against 22 to 29), a starved station (§3) makes
nothing, and a seat at war builds Armies where it built Power Plants. Nothing was run with one rule
alone reverted.

---

## 1. The sea reaches inland: one inland slot turns coastal at every rise, wall or no wall

*Ticket [#276](https://github.com/whaleyjoshua2/Dying-Earth/issues/276).*

Every Sea Level rise that reaches a Region, after it has taken that Region's coastal slots (or been
held off by a working Sea Wall), turns **one inland slot coastal**. The wall holds the taking off,
not the turning: behind a standing wall the rise takes nothing and the coast still moves one slot
further in. The slot turns *after* the taking, so it faces the next rise and not the one that made
it. An **empty inland slot turns first**; when none is empty the **oldest** inland Facility turns
with its slot, the order the coast already drowns in. A slot a queued build has reserved is not
empty, so the build turns with it and completes on the coast. There is no ceiling: a Region may turn
all coast, one slot a rise, until nothing inland is left, and then a rise turns nothing. A slot
added by a raise of the Industry Level turns like any other.

There is no landlocked flag, because every Region on the board has a coast: a Region at Coastal
Exposure 0 would be spared, and none exists. The strip's coastal and inland boxes and one Report
line are the whole of what the player sees: *"The coast now reaches one slot further in."*, or
*"… further in: a Research Lab stands on it now."* when a Facility turned with its slot. The count
of turned slots is saved with a default of nought.

Measured over the closing sweep: the coast no longer runs out, so the sea takes more and walls are
worth more. Coastal slots lost a game rose in every seating where the game runs long (27, 34 and 36
against 22, 25 and 29); 50, 53, 52 and 14 inland slots turn coastal a game; Sea Walls built over a
batch rose to 557, 486, 478 and 403 from 478, 330, 312 and 397, and walls standing at the end to 197,
207, 207 and 176 from 184, 134, 148 and 168.

## 2. The Greenwash: Ducats and Influence spent to lower your own Blame

*Ticket [#277](https://github.com/whaleyjoshua2/Dying-Earth/issues/277).*

A **Greenwash** is the Smear's mirror, worked on one's own ledger: **2 ppm off per Influence, and a
Ducat beside every Influence spent**, so cleaning your name costs strictly more than dirtying a
rival's and half a Neutral credit's Ducat price per ppm. It comes off the whole ledger, floored at
nought, and no gram of CO2 leaves the air. One campaign a turn, any amount the Allotment and the
Ducats cover; the Custodians may, like anyone. It is public and no offence: *"The Prospectors
greenwashed: 10 ppm off their Blame."* The control sits on the player's own page of the Faction
window, between Relations and Holdings, and the Blame sentence gains a sixth clause, *"N cleaned by
campaign"*. *Propaganda* stays on the Smear's avoid list; the glossary's **Greenwash** entry avoids
propaganda, PR, spin, whitewash, clean-up and self-smear.

A computer seat greenwashes at weight 4 (`greenwash` in `ai.toml`) when its own share stands above
the fair quarter, carbon credits are not to be had (no seller, nought offered, Hostile, or past its
purse), and it keeps 20 Ducats past the price; where credits are to be had it buys them instead,
never both in a turn. Measured: only the Prospectors ever greenwash, a median 70 to 90 ppm a game in
the three seatings where they are dirty; the two levers add rather than substitute (§10).

## 3. A Blockade that starves, and a Blockade that is ordered

*Ticket [#278](https://github.com/whaleyjoshua2/Dying-Earth/issues/278).*

**A Blockade is a stance**, ordered, never a side effect of presence. This reaches back to 0.07.0:
unloading, refuelling and building into a slot are now shut only by a stack on Blockade; a warship
sitting in a slot on Hold blockades nothing. The stance is Ships only, refused unless a warship of
the seat's sits in a slot that is not its own station's; a blockading stack neither attacks nor
intercepts.

Two rules starve. A **station starves under a Blockade of its slot**; a **Colony on the ground
starves while one rival holds Orbital Control of its Body outright and has a stack there on
Blockade**. A contested orbit starves nobody. Starved means silenced: output, Research, the Trade
Post's Ducats, the Relay's Allotment; untouched are the Habitat's room, the Core Module, the
Shipyard, the Barracks and the Archive's fund. **Upkeep is still paid.** Nobody dies and nothing is
destroyed. Read live at each Income. Each Colony-turn of it is the 0.08.2 spec's weight-1 offence
against the holder. The card reads *"Blockaded by the Prospectors: producing nothing, upkeep still
paid."* and on the ground *"Under the Prospectors' Orbital Control: …"*; the Report says *"ISS over
Earth is blockaded by the Prospectors: it made nothing this turn, and its upkeep was paid."* The
computer is offered the stance where its warship sits in a rival station's slot at weight 3 for
every Faction (`stance_blockade`), and wants a warship where a Colony of its own is starved. No
counter this version: **the orbital Battery goes to the map's fog**.

Measured: the Blockade is the Prospectors' alone, and only where they go second, 139 Colony-turns
starved over that batch (the Archivists' 61, the Arkwrights' 49, the Custodians' 29) and 18 where
they go first; nought in the other two seatings.

## 4. Battles pollute

*Ticket [#279](https://github.com/whaleyjoshua2/Dying-Earth/issues/279).*

A Battle on Earth, in Earth orbit or at a Colony on Earth puts carbon in the air: **0.5 ppm per hit
landed, 2.0 per building burned** in the rolls after a ground Battle or on a taking
(`war_ppm_per_hit`, `war_ppm_per_building` in `climate.toml`). It reaches **the stock and the
ledger both**: a new **War** source on the emissions breakdown reaching the CO2 stock, and each
Faction party's share written to its Blame as emitted. Every party with a seat wears the ppm of its
own hits; a neutral Region's Army's hits are nobody's; two aggressors share by what each did;
buildings burned on a Pacified or completed Occupation are charged to the taker, and a place taken
by Influence rolls nothing and pollutes nothing. Mars orbit fouls nobody's air. It is charged at the
next Climate phase from a bucket like the Wildfire's, and **counts against a Stabilization run**.
The Climate Panel carries a **War** line when above nought, the rule on hover; the sweep prints it
by seat and nobody's.

Measured: war ppm a game, median by seat, 8 for the Prospectors where they hold East Asia and 6 for
the Arkwrights where they do; nobody's share is nought in every seed of eighty, because no Battle is
against a neutral (§7). Against a Prospector ledger of 586 to 669 ppm, war is a signal, not a lever.

## 5. Buildings that say what they do

*Ticket [#280](https://github.com/whaleyjoshua2/Dying-Earth/issues/280).*

Every building with no resource output says what it does where *"no output"* was drawn, in **one
clause, two at most**, as **static prose from the data**: a `does` field on the building's row in
`facilities.toml` and `modules.toml`, drawn on all four surfaces (card row, hover, build menu, order
list) from the one sentence. **No new glyph**: the upkeep keeps its Energy glyph and the effect is in
words; the live figure stays on the card's own line. Sixteen sentences: every "no output" row, plus
a clause for the five Uniques (Investment Bank, Reactor, Academy on Earth and off it, Spaceport,
Exchange), and the Scrubber's and Sea Wall's hand-written hovers moved into the same field, which
found the Scrubber's hover had said 4 Energy upkeep for two versions while the data said 3. A row
without a `does` reads exactly as before.

## 6. The Battle Report: Battles by name, as a line and as a Moment

*Ticket [#281](https://github.com/whaleyjoshua2/Dying-Earth/issues/281).*

**Every Battle is a line of the Report at its place**: ranked 4, with a Ship destroyed, when a unit
died; unranked when nobody lost one; the block at the foot stays as the detail. The Battle's line is
written before the losses are applied so it headlines over the unit's own line. **The Battle's
Moment fires when any unit is destroyed**, an Army included, and names it; buildings burned on a
taking keep a Moment under their own name, **A place taken by force**, the tenth Moment kind. The
party line reads the Ship's and the Army's names, a neutral Region's Army as *the 1st Egyptian
Army*, and what each took: *TSV Valiant took 2 hits; PMV Aurora escaped after 3 hits; PMV Magellan
destroyed*. An Army destroyed writes a line by name as a Ship has since 0.08.1. The line and the
block say the **first-round odds** the attacker faced, labelled as such; several attackers are named
together and the first's odds quoted. The Battle record gains a real place, saved with a default.

## 7. Neutral states arm when threatened

*Ticket [#282](https://github.com/whaleyjoshua2/Dying-Earth/issues/282).*

A neutral Region that is **threatened** raises a **Levy**: a second Army, standing, never marching,
named from its home (*the 2nd Indian Army*) and marked *levy* on the card, at **Industry Level +
2**. Threatened means a built Army of any Faction in a neighbouring Region, or a neighbour under
Occupation, **stance-blind**, so nobody's orders are disclosed by it. A Levy is raised only while
Unrest is under the Standing Army's threshold (a restive state musters nothing), one per Region; it
**stands down** at the Income after the threat has passed, or the moment the Region becomes
somebody's. A neutral that is attacked and holds — a defender of its own still standing after the
Battle, unescaped, with strength above nought — gains **+1 to its Standing Army for good**, to
Industry + 4; the earned step never lifts. Neutral Regions only. Healing runs only while Unrest is
under the threshold; **Agitate stays illegal on neutrals**. A destroyed Standing Army returns **two
Incomes later**, not the next, for every Region, settling an assumption the code had carried since
ticket #50. The card's Army rows carry the rule on hover; the map shield sums the Armies' strength
and rises with the Levy. Three Report lines; a **Levy** glossary entry.

Measured: **3 Levies raised and 0 attacks held against in eighty games**. A finding, not a defect:
every Region is taken by Influence in the first turns and the computer's first Army appears around
turn 20, so no neutral Region is standing to be threatened. The rule fires on a board with a neutral
beside an Army, as the test and the picture show; it will matter to a human who marches early.

## 8. Yields in glyphs on the planet card, and a founding door without the planet-wide line

*Ticket [#283](https://github.com/whaleyjoshua2/Dying-Earth/issues/283).*

The planet card gains a **Colonies and sites block**: one row per Colony on the ground and per open
site, the slot's glyph row beneath, clicking a row selects it; on Earth the block lists standing
Colonies only. **Stations keep their rows without yields.** The Colony card carries the glyph row
under its heading. **The planet-wide line moves to the head of the planet card and off the founding
door**, reversing #258's *"keep it"*: when founding from a Ship the player sees the sites, not the
Body as a whole. The Antarctic "by sea" door wears the founding button's face with the site's glyphs;
the two Ship doors are unchanged. Every new row goes through the glyph-row helper, never the prose
renderer, so #258's trap cannot recur.

## 9. The computer seats may attack a Region they did not lose

*Ticket [#284](https://github.com/whaleyjoshua2/Dying-Earth/issues/284).*

**The Prospectors' rule for all four seats**: a computer seat marches on a neutral Region, or a
Region a rival holds, when the first-round odds clear the bar. A rival's Region needs **a cause**:
the seat is **Cold or worse toward that rival** (`war_cause = -5` in `ai.toml`, the Smear's gate),
the AI's first reading of Relations for war; a neutral needs none; a place lost to a running
Occupation may be retaken. **The attack is the stance by condition**: when allowed and the odds clear
the bar, Attack is the candidate and Hold is not, on the ground and in orbit, so the score contest
that let Hold win every tie is gone. In orbit the cause is Cold or worse toward the seat holding
Orbital Control against you, or toward any enemy present when nobody holds it, beside the old
blockade-breaking allowance. **The player is a target on the same terms**; the cause is the shelter.
One new war a turn per seat, marches on neutrals uncounted. The Army cap stays; Carriers stay the
Prospectors' alone. **An occupier stays** (holds at three times the weight, marches nowhere), and the
Battle line and the Occupation read one predicate, so an escaped attacker is never promised an
Occupation.

Measured against the 0.08.4 rules (§11): Battles over eighty games rose from 113 to 172, and the
seat the rule freed is the Arkwrights, who at 0.08.4 built 60 Armies where they go first and fought
no Battle with them, and now fight 52. The Custodians and Archivists build Armies (13 and 34 over the
batch) and open 7 and 3 Battles. Occupations broken fell from 20 to 2, since an occupier stays.

## 10. Carbon credits move from the Trading window to the Faction window

*Ticket [#285](https://github.com/whaleyjoshua2/Dying-Earth/issues/285).*

The **Offer block sits on the Custodians' own page** of the Faction window, beside the Blame line
whose credit it sells and the Greenwash; the **Request block sits on the Custodians' page** for
everyone else, under the Accords and the Smear: *Request n ppm for {cost} Ducats*. **The word changed
and the rule did not**: a request is filled at End Turn from the standing offer or refunded where it
ran out. The Trading window's fourth line is gone; the computer seats offer and buy as before; the
refusals keep their words; the blocks show only when a Custodian seat sits at the table.

Measured: the credit trade rose, not fell. Bought over a batch: 1214, 1740, 922 and 680 ppm against
733, 1408, 855 and 630 at 0.08.4; the Custodians sell every credit they offer, as before.

## 11. The sim and the sweep count the war

*Ticket [#286](https://github.com/whaleyjoshua2/Dying-Earth/issues/286).*

Every military figure is a counter on the game, incremented at the event, never a sentence scraped
from the log: Battles by aggressor and against a neutral Region's own Army; Armies and warships
built and lost, by seat; Standing Armies lost; Occupations begun and broken; places taken by force
(an Occupation completed or Pacified) beside places taken by Influence; marches on neutral and on
held Regions; attacks in orbit. The counters ride through the save. The sweep prints them as two
lines directly under *Places taken by Influence over the batch*; the `sim` example prints them per
seed. Blockade-turns suffered and imposed (§3), war ppm by seat and nobody's (§4) and Levies raised
and attacks held (§7) print where their tickets put them.

The map asked for this ticket to be built first so the 0.08.4 rules had a military figure before
any military ticket was judged; it was built last. The baseline was measured afterwards by porting
the counters onto `main` in a throwaway worktree and running the same sweep:
[`sweeps/baseline-0.08.4-military.txt`](../dev-diary/2026-09-20-version-0.08.5/sweeps/baseline-0.08.4-military.txt),
whose win column reads 26 / 28 / 7 / 0 and 19 collapses, the recorded baseline exactly.

---

## What the closing sweep says

Run as `sweep 20 --balance --seatings --steps=300`; the output is
[`docs/dev-diary/2026-09-20-version-0.08.5/sweeps/final-0.08.5.txt`](../dev-diary/2026-09-20-version-0.08.5/sweeps/final-0.08.5.txt),
read against
[`final-0.08.4.txt`](../dev-diary/2026-09-19-version-0.08.4/sweeps/final-0.08.4.txt) and the
military baseline above.

**The win column is in the header of this document.** By seating, seat 0 first: 0 / 16 / 0 / 2
against 0 / 15 / 0 / 0 with the Custodians first; 9 / 5 / 0 / 0 against 13 / 0 / 0 / 0 with the
Prospectors first; 9 / 3 / 1 / 0 against 7 / 6 / 0 / 0 with the Arkwrights first; 0 / 20 / 0 / 0
unchanged with the Archivists first. Collapses 2, 6, 7 and 0 of 20 against 5, 7, 7 and 0.

**The military block, against the 0.08.4 rules.** Over eighty games:

| | 0.08.4 rules | 0.08.5 |
|---|---|---|
| Battles opened | 113 | **172** |
| … by the Prospectors | 113 | 110 |
| … by the Arkwrights | 0 | 52 |
| … by the Custodians / Archivists | 0 / 0 | 7 / 3 |
| Battles against a neutral | 1 | 0 |
| Attacks in orbit | 0 | 63 |
| Armies built (Prospectors / Arkwrights / Archivists / Custodians) | 129 / 64 / 27 / 17 | 122 / 66 / 34 / 13 |
| Armies lost | 19 | 20 |
| Standing Armies lost | 43 | 47 |
| Warships built / lost | 126 / 0 | 129 / 0 |
| Occupations begun / broken | 69 / 20 | 67 / 2 |
| Places taken by force | 39 | 51 |
| Places taken by Influence | 1374 | 1317 |
| Blockade Colony-turns imposed | not counted | 157 |
| Levies raised / attacks held | none existed | 3 / 0 |

- **The war is the East Asia seat's.** Whoever starts there with the Ducats fights: the Prospectors
  opened 57 and 49 Battles where they hold it, the Arkwrights 52 where they do. Every other seat in
  every seating opened fourteen between them. The attack rule (§9) did not make the timid seats
  fight; it let the Arkwrights use the sixty Armies they were already building. Attacks in orbit
  went from none to 63 because a warship in a rival's slot is now offered Blockade and Attack
  without Hold swallowing both.
- **No Battle is against a neutral, so nobody's war ppm is nought and the Levy fired three times.**
  Every Region is somebody's by the time any Army exists. §4 and §7 are rules for a human who
  marches early; on the computer's board they are inert this version.
- **The computer never loses a warship**, at either rule set: 126 and 129 built, none destroyed.
  The computer's warships blockade and are not met by a warship that fights back.
- **The sea** takes more (27, 34, 36 and 16 coastal slots a game against 22, 25, 29 and 17) and
  drowns fewer Facilities (10, 9, 11 and 5 against 12, 13, 17 and 6), because the slots it takes are
  more often empty ones the coast has just reached; walls are built more and stand more (§1).
- **Blame.** The Prospectors' ledger at the end fell from 813 to 669 where they go second and 815 to
  586 where they go first; their share from 0.71 and 0.73 to 0.65 and 0.63. Two levers add: credits
  bought rose in every seating (§10) and the Greenwash takes 70 to 90 ppm a game on top (§2). The
  Smear they wear rose to 170 from 140 in one seating and fell in the others. **The credit trade did
  not fall.**
- **Ducats.** The seat at war pays for it: the Prospectors made a median 1053 a game where they go
  second against 1575, and 3580 where they go first against 4573; the Custodians made 1099 in the
  first seating against 697. The Fund still clears 2500 where the Prospectors hold East Asia (2534
  and 2507 median) and the Archivists' two wins came in the seating where the Prospectors' Ducats
  went to Armies.
- **Relations run colder again.** 133, 109, 145 and 103 of 240 ordered pairs end at Cold or worse
  against 117, 106, 133 and 109; 51, 45, 62 and 46% carry a scar against 40, 41, 59 and 48%. War,
  the Blockade and the Smear are all offences.
- **Which seat the sum landed on: the Prospectors**, by Ducats rather than by Blame or by Battle.
  They build the Armies and the warships, wear the Smear, buy the credits and greenwash, and they are
  the only seat that ever blockades; the two games they lost went to the Custodians and the
  Archivists. Nothing landed on the Archivists beyond the Blockade, and the Blockade is confined to
  one seating.
- **Nothing was re-fitted on this sweep.** The candidates the sweep names: `stance_blockade` (139
  Colony-turns in one seating, none elsewhere), `war_ppm_per_hit` (a signal against a 600-ppm
  ledger), and the build-Army weight for the Custodians and Archivists, which the map's fog holds
  for the balance version. The designer, shown the column and the three: **ship** and **none**.
- **The kit.** This version ships as the Windows kit alone, built on this machine
  (`dist/dying-earth-0.08.5-windows.zip`); the designer's word on the closing ticket was *"just the
  windows kit done locally no need for linx at this time"*. The Linux kit was built by the
  release-kits workflow before that word and is not shipped.
