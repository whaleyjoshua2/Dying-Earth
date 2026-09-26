# Dying Earth — version 0.09.1, the plain-speaking version: nukes and a Sink that keeps what it is given, a bonus for being first to a Body, Battles that cost Fuel, the Archivists' Condition rebuilt, every Faction's gate chain in its own list, the game telling you what it knows, seven defects, and the orbital war brought to life

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.09.1](https://github.com/whaleyjoshua2/Dying-Earth/issues/342), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-09-24-version-0.09.1/`](../dev-diary/2026-09-24-version-0.09.1/).

**What the version is.** Version 0.09.0 answering for itself after its first real playtest, and then
answering for what the computer seats could not do. Four new rules the designer listed: **nukes**, and
what firing one does to the Natural Sink (§1); an **Influence bonus for being first to settle a Body**
(§2); **Battles that cost Fuel** (§3); and **the Archivists' Condition rebuilt** around 125 Research
(§4). One repair to the computer's head, **every Faction's gate chain in its own pick list** (§5). The
game now **says what it knows**: a rival closing on a place you hold (§6), your own Victory Condition
(§7), the lights about to go out (§8), what a building makes and why (§9), and a shut door that names
the move that opens it (§10). **Seven defects** the playtest found are fixed (§11). Then the work the
build uncovered: a Launch Site that **reaches any orbit** (§10), a **shut Barracks and a shut Habitat**
(§12), **Influence paid as shown** (§13), a quieter Report (§14), the **orbital war** made to happen
(§15), and the **Archive** made buildable (§16).

**What it did to the win column** (80 games, per Faction, at the shipped climate cell):

| | 0.09.0 | 0.09.1 |
|---|---|---|
| Custodians | 5 | 6 |
| Prospectors | 31 | 28 |
| Arkwrights | 1 | 3 |
| Archivists | 0 | 2 |
| collapses | 42 | 41 |

---

## 1. Nukes, and what firing one does to the Natural Sink

The authority is [ticket #343](https://github.com/whaleyjoshua2/Dying-Earth/issues/343), which folded
in #344.

The **Missile Carrier**, a fifth Ship type that cannot fight, carries one **Warhead**, spent by firing
and reloaded only at a Shipyard of its own. It is behind the game's first weapon Tech, **Missile
Technology** (Propulsion, rung 3, after Hardened Hulls). A **Launch** guts a Region, a ground Colony or
a station a rival holds: every building rolls 60%, the Core Module and the Archive spared, 40–60% of
the people die, and at a Region the Standing Army dies and the Industry Level falls by one. It is
rung 4 against the holder, and it may be given **over Earth**, where a Bombard is refused. On Earth
alone it fouls the air and raises the **Natural Sink by 0.25 for good**, and **The Sink Weakens now
subtracts 2.0** instead of setting the Sink to 4.0, so a Sink somebody raised keeps what it was given.
The carrier's frailty under the escort rule is its counter; there is no anti-missile rule.

## 2. An Influence bonus for being first to settle a Body

The authority is [ticket #345](https://github.com/whaleyjoshua2/Dying-Earth/issues/345).

Founding a **ground Colony** on a Body off Earth where none has ever stood claims that Body's
**first**, once and for good. A station claims nothing, Antarctica counts for nothing, and Venus can
never be claimed. The first Colony's **Core pays its founder +1 Influence a turn** while they hold it,
sleeping under a rival and waking on a retake, and a **one-off windfall** is paid: 5 at the Moon, 15 at
Mars, 20 at Phobos and Deimos. Both sit outside the Faction multiplier. A tie goes to the contested-slot
rule.

## 3. Battles that cost Fuel

The authority is [ticket #346](https://github.com/whaleyjoshua2/Dying-Earth/issues/346).

A Battle in orbit takes **2 Fuel from every Ship in it**, once, fought or not, on either side; a
Battery pays nothing. A hull that could not pay **fights at half strength** until it refuels. A
warship must hold the charge to hold Orbital Control, contest an orbit, blockade or intercept.

## 4. The Archivists' Condition: 125 Research, and the quarter-cap gone

The authority is [ticket #347](https://github.com/whaleyjoshua2/Dying-Earth/issues/347).

**125** is one figure for the Archive's Research and the Archivists' first Victory bar; the loader
refuses a table where they disagree. The quarter-cap is gone, so Research banks freely from turn 1
and the Victory figure is no longer clamped. See §16 for the Archive itself.

## 5. Every Faction's gate chain in its own pick list

The authority is [ticket #348](https://github.com/whaleyjoshua2/Dying-Earth/issues/348).

Each Faction's computer pick list opens with its own gate's antecedents, and the shortlist always
carries the **next rung of the Lead's chain** (or its gate, once reachable), for a human Lead as much
as a computer one. The Prospectors lose both denial levers. **Hardened Hulls and Missile Technology
close all four lists**, behind the gate chains ([#355](https://github.com/whaleyjoshua2/Dying-Earth/issues/355)).
[Ticket #360](https://github.com/whaleyjoshua2/Dying-Earth/issues/360), *the cheapest Tech is always
the right pick*, was measured and closed as already fixed by this section.

## 6. A warning when a rival closes on a place you hold

The authority is [ticket #349](https://github.com/whaleyjoshua2/Dying-Earth/issues/349).

A held place is **Pressed** while any rival's Standing is within 10 of yours. The Command Cluster lists
every Pressed place under the Influence slider, three and then *"and N more"*, never naming the
rival; a click goes there. The card's challenger line turns amber on the same test.

## 7. The player is told their own Victory Condition

The authority is [ticket #350](https://github.com/whaleyjoshua2/Dying-Earth/issues/350).

The turn-1 Report line names the player's own Condition, in a short per-Faction clause whose figures
are filled from the Victory bars.

## 8. An alarm before the lights go out

The authority is [ticket #351](https://github.com/whaleyjoshua2/Dying-Earth/issues/351).

The top bar's **Energy figure turns red** while the next Income would shut anything, and its hover
names what goes dark, in order. It does not block End Turn. The Natural Sink's loss is said on the
Report line after the fact.

## 9. A hover that explains what a building makes

The authority is [ticket #352](https://github.com/whaleyjoshua2/Dying-Earth/issues/352).

**Every multiplied figure's hover is its arithmetic**, computed through the same chain the rule runs:
*"2 base / × 1.32 for 1.45B people, weighted by Education / × 1.10 for Education 1.10 / × 1.25 as the
Custodians / = 3.63, rounded down to 3"*. On slot boxes, Facility rows, build buttons and Module tiles.
The Education Level hover says it counts twice in a Research Lab. A chained hover drops the upkeep and
Emissions sentences to stay within six lines.

## 10. A Launch Site reaches any orbit, and a shut door says the move first

The authorities are [ticket #357](https://github.com/whaleyjoshua2/Dying-Earth/issues/357) and, for
the Pioneers block, [ticket #356](https://github.com/whaleyjoshua2/Dying-Earth/issues/356).

**A lift from a Region with a working Launch Site reaches any orbit of Earth**, Colonists and Armies
both; a ground Colony, with no Launch Site, is reached from low orbit alone. Every refusal a change of
orbit cures reads **"Move this Ship to {orbit}, then {act} next turn: {rule}."** A shut door's hover
is the refusal alone. On a Region card the **Pioneers block stands above the Facilities heading**.

## 11. Seven defects the playtest found

The authority is [ticket #353](https://github.com/whaleyjoshua2/Dying-Earth/issues/353).

A build refusal names whether a Shipyard is absent, still building or shut; a founding names the slot;
a partial unload and a clamped lift say who stayed; a lift fills as far as the room goes; a Research
Directive names what it bought; a quiet turn's Report falls through to a headline; the driver's help
names the Archive's rules.

## 12. A shut Barracks, and a shut Habitat

The authority is [ticket #359](https://github.com/whaleyjoshua2/Dying-Earth/issues/359).

A **mothballed or dark Barracks raises and repairs no Army**; a garrison already standing stays. A shut
**Habitat still houses its people**, but while it is **occupied** (more Colonists than the working
Habitats and the Core can hold) the Colony makes **everything but Energy at half**, once.

## 13. Influence paid as shown

The authority is [ticket #358](https://github.com/whaleyjoshua2/Dying-Earth/issues/358).

The Allotment reads each Module's own figures, so **Relay Networks and the Chorus are paid**. The
Chorus's per-Colonist Influence sits **outside the Faction multiplier**. Climate charges each Facility's
Emissions from the figure its card shows, Provisional Findings included.

## 14. The Report tells only a fall into a worse level

The authority is [ticket #362](https://github.com/whaleyjoshua2/Dying-Earth/issues/362).

*"Have not forgiven"* is retired. The Report tells the player only of a pair **involving them** that
**falls into a worse named level**, both ways, folded one line each: *"The Prospectors are now Cold
toward you, the Arkwrights Wary."* / *"You are now Wary of the Archivists."*

## 15. The orbital war

The authorities are [ticket #355](https://github.com/whaleyjoshua2/Dying-Earth/issues/355) and
[ticket #363](https://github.com/whaleyjoshua2/Dying-Earth/issues/363).

**Rule: a working Battery opens a Battle on a rival warship on Blockade in its own orbit.** **Rule,
repaired:** a Ship takes one order a turn both ways; a Change orbit, Transit or Refuel is refused after
a Launch, Rearm or Bombard. The computer seats now contest an orbit **with cause**: cause builds a
fleet, a Blockade is the stance, a blockaded station buys a Battery, the Attack is read orbit by orbit,
carriers are built and stay to fire. Warship weights: Arkwrights 4, Archivists 3.

Against the designer's bar of 10 games of 80 each: Blockades in 20, Launches in 21, orbital Battles in
7 (**not met**).

## 16. The Archive

The authority is [ticket #361](https://github.com/whaleyjoshua2/Dying-Earth/issues/361).

**Rule: The Upload gates the Archivists' win, not the Archive's order**, as before #199. The computer's
Archivists ferry people off Earth and build room at the Archive's Colony. The Archive is ordered in 12
games of 80 (from 6) and completed in 8 (from 6). **Not met:** it completes at a median turn 33, and
uploads reach 4 against the 12 the second part asks.

---

## What the closing sweep says

`sweep -- 20 --seatings --balance --steps=300 --sinks=6`, eighty games, the per-Faction totals read at
the foot of [`sweeps/final-0.09.1.txt`](../dev-diary/2026-09-24-version-0.09.1/sweeps/final-0.09.1.txt).

- **Wins** Custodians 6, Prospectors 28, Arkwrights 3, Archivists 2; **collapses 41** (0.09.0: 5 / 31
  / 1 / 0, collapses 42). The two Factions that won nothing now win something; the Prospectors still
  take over a third of all games.
- **Victory gates completed** 73 / 58 / 50 / 40 of 80, median turns 25 / 28 / 26 / 28.
- **The orbital war** exists: Blockades in 20 games, Launches in 21 (39 in all), orbital Battles in 7 (11 in all, one off Earth).
- Collapses moved within the version: 42 → 35 (#348) → 39 (#357) → 33 (#359) → 35 (#363) → 41 (#361,
  from the Archivists' ferry). A rising collapse rate is reported as a figure.
