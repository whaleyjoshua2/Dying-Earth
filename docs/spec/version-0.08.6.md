# Dying Earth — version 0.08.6, the defence version: a Region defended by its people, one Army system, Dig In, a disengage nudged down, a place taken whole, an Occupation that must be held, an Army that attacks the turn it lands, two Colonists on every starting station, and four pieces of interface

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.08.6](https://github.com/whaleyjoshua2/Dying-Earth/issues/289), and the
pictures and measurements that decided it are in
[`docs/dev-diary/2026-09-21-version-0.08.6/`](../dev-diary/2026-09-21-version-0.08.6/).

**What the version is.** Version 0.08.5 made the war real and ended with 172 Battles over eighty
games, every one of them the East Asia seat's, 67 Occupations and 51 places taken by force. This
version is the ground war's second half, taken from the ground review of 2026-09-19: the defender
gets a say. A Region's Army reads its Industry and, while it defends, its police and its calm (§7,
§12); an Army may **Dig In** (§8); the escape that decided Regions is rarer (§6); a place Pacified is
taken whole and a Battle burns nothing (§9); an Occupation that breaks costs (§10); an Army fights
the turn it lands (§11); and the four kinds of Army become one system (§12), which the designer
added to the map after seeing the first two of those built. Beside the war: two Colonists on every
starting station so it can build from turn one (§1), and four pieces of interface asked for in the
same breath (§2, §3, §4, §5).

**What it did to the win column**, 20 seeds across four seatings at the shipped climate cell
(sink 6, step 300), against the 0.08.5 baseline:

| Faction | 0.08.5 | 0.08.6 |
|---|---|---|
| Custodians | 28 | **43** |
| Prospectors | 26 | **21** |
| Arkwrights | 9 | **2** |
| Archivists | 2 | **0** |

**The column moved a long way, toward the Custodians, and the war is gone.** Every rule on this map
that touches the ground hardens the defender, and the seat that attacked was the Prospectors: over
eighty games the computer marched on a held Region **5 times where it marched 91**, began **2
Occupations where it began 67**, and took **2 places by force where it took 51**. Battles fell from
172 to 62, all but five in orbit. With no conquest the Prospectors' Ducats fell where they go first
(614 a game against 3580) and the Custodians' rose in every seating (1251, 1912, 5330 and 214
against 1099, 1082, 4212 and 222 for the seat in the same chair). The Custodians win 43 of 80. The
Arkwrights, at two, and the Archivists, at none, lost the games the Prospectors' war used to hand
them. Which seat the sum landed on: **the Prospectors**, by the war they can no longer win, and by
the Ducats that went with it; and on the seats at the bottom, whose two and nine were the war's
by-product. Collapses **14 of 80** against 15, but the Arkwrights-first seating collapses **12 of
20** where it collapsed 7, at a median end of +3.01 C; the other three end cooler.

The designer, shown the column and the three re-fit candidates (the two defence points and the
threat steps in `units.toml`, the computer's 60% attack bar, the Ducat side of the opening):
**ship**, **re-fit none now**, and the version ships as the **Windows kit alone**, built on this
machine (`dist/dying-earth-0.08.6-windows.zip`); the designer's word on the closing ticket was
*"ship, none now, just windows"*. The balance is the next version's, with this sweep as its
baseline. No Linux kit was built.

---

## 1. Two Colonists on every starting station, and two Pioneers for the Arkwrights

*Ticket [#290](https://github.com/whaleyjoshua2/Dying-Earth/issues/290).*

The ISS, Tiangong and Axiom open with **two Colonists aboard**, from nowhere; no Region is debited.
The point is that a station can build from turn one: a Colony's Module cap is one slot per Colonist
with no base, so a bare station had no slot at all, and the three seats with a station could build
nothing there until people were lifted. Two aboard is two slots free and two berths left in the Core
Module. The **Arkwrights**, who have no station, open with **two Pioneers waiting in their start
Region**, a gift outside Coach Class that takes no population; they wait for the first Ship.

The tutorial is six notes, one ask a turn: spend Influence, **build a Habitat on the ISS**, recruit
four, send them up, then the climate and the Research race. The computer plays an opening: while a
starting station has a slot free and under four berths empty, and until its first Habitat stands or
is on order, the Habitat is pushed first, not while Energy is tight. Written without the once-only
clause it fired every time the muster refilled the station and the Prospectors put Habitat after
Habitat on Tiangong: 330 Ducats a game against 2287 with the rule off.

The ISS is dearer to take, 40 where a bare one read 20, since the threshold counts Colonists. The
sim and the sweep print Modules beyond the Core on a starting station at the end of turn 3; in the
closing sweep every seat with a station has two in every seed but the Prospectors where they go
second, who have one.

## 2. A building shown in its box the moment it is ordered, and marked while it is built

*Ticket [#291](https://github.com/whaleyjoshua2/Dying-Earth/issues/291).*

An ordered building shows in its box **the moment the order is placed**, before End Turn, on the
Region card's slot boxes and on the Colony and station cards' Module tiles alike. The box keeps its
hatch but is dimmed like a mothballed tile, with *ordered* in one corner and the turns to complete
in the other; once the turn ends the word becomes *building* and the count runs down. **Right-click
on an ordered box cancels the order**, through the same cancel the orders list's button uses; a
building under way cannot be cancelled and stays unclickable. An ordered Facility is drawn in the
coastal or inland group its slot will take, by the rule the order uses, and the Facilities header
subtracts ordered slots: *4 of 9 slots free, 1 ordered this turn*.

## 3. The Trading window opens below the top bar

*Ticket [#292](https://github.com/whaleyjoshua2/Dying-Earth/issues/292).*

The Trading and Victory windows open **under the top bar** and to the right of the Faction window's
home, where they took egui's corner fallback over the bar's figures. The bar's foot is **measured
once a frame** and every window that avoids the bar reads it: the Faction window, the Tech Tree, the
Climate Panel, Trading and Victory; the three literal guesses at its height (120, 120 and 104) are
gone. The Tech Tree's scroll bound is measured at the line the tree starts on, since with the guess
retired its old bound overshot an 800-pixel screen and egui shoved the window up over the bar. A
**top bar** entry joined the glossary.

## 4. The Smear and the Greenwash on sliders

*Ticket [#293](https://github.com/whaleyjoshua2/Dying-Earth/issues/293).*

Each campaign's amount is set on a **full-width rail of fixed width, in single Influence points,
from 0 to the turn's whole Influence** (the Allotment plus what was bought), with the part already
committed to other orders painted in nobody's grey from the right, so the scale holds still through
the turn. The Greenwash's Ducat bound is painted as a second, bluer region and the line under the
rail names which bound bites. The button stays and reads the rail; the number field is gone. Both
blocks read the command cluster's figure, so **Influence bought this turn counts**, which the old
fields ignored. No cap in data.

## 5. The command cluster: a slider for the spend, Max kept, a tenth larger, and End Turn a sun

*Ticket [#294](https://github.com/whaleyjoshua2/Dying-Earth/issues/294).*

The spend is set on the same rail the Smear uses; the Spend button names the amount and the place.
**Max** places the order for everything left and moves the slider to the bound. The cluster's scale
constant is **1.265**, a tenth over 0.08.1's 1.15. **End Turn is a sun**: a disc painted by hand,
gold with a darker limb, four rings brightening toward a light above and left, five fixed sunspots,
the words *End Turn* beneath and the Enter key on the hover, at the right edge of the last row with
Max and the every-turn tick on its left; softened and grown a tenth at the designer's word on the
first picture. It dims to embers while a Tech pick is owed. The spectator's End Turn on the top bar
stays a rectangle.

## 6. The disengage chance nudged down

*Ticket [#295](https://github.com/whaleyjoshua2/Dying-Earth/issues/295).*

After every round of a Battle a damaged unit leaves with chance **damage over hit points over
three**, where the First Playable wrote two into the code and never tuned it; the figure is
`[disengage] divisor` in `units.toml`. An undamaged unit never leaves; **Evade stays a flat half**;
both sides roll alike. The sweep counts units escaped by seat and Battles with an escape, and the
baseline was printed first: under the old figure and the old strength, 12 units escaped and 11 of 37
Battles had an escape on the Custodians-first seating; with the nudge alone, 5 and 5 of 35. Over the
closing sweep 5 units escaped from 62 Battles.

## 7. A Region's defence is its people

*Ticket [#296](https://github.com/whaleyjoshua2/Dying-Earth/issues/296), as rewritten by §12.*

As built on this ticket: a Standing Army's strength was Industry + 1, plus the earned steps, **plus 1
for a working Constabulary and plus 1 while Unrest is under 4**, with hit points equal to the whole;
a Levy the same two on top of Industry + 2; built Armies unchanged. The designer first answered
"apply it to built armies too" and then set that aside, and §12 settled it the other way. The rule
that stands is §12's: the two points are **defence**, not body. What this ticket keeps: **hit points
equal strength**, read live, and an Army whose damage reaches its strength is **destroyed**, at
Income as in a Battle, which closes the limbo where a Standing Army at strength nought neither
fought nor died. The computer's Evade trigger reads the live hit points.

Measured on this ticket alone, the two points stopped the computer's ground war outright on the
Custodians-first seating: marches on held Regions from 18 to 0. A single 4 never cleared its 60% bar
against a 5 or 6 with the hit points to match. That measurement is why §12 was asked for.

## 8. Dig In: a third live stance for Armies

*Ticket [#297](https://github.com/whaleyjoshua2/Dying-Earth/issues/297).*

A dug-in Army fights **two stronger while defending** and **never rolls to disengage**; its hit
points do not follow the bonus. While its stance is Dig In a march order and a Carrier lift are
refused; the stance must be changed first, and the change takes effect at Resolution, so digging out
costs the turn. It takes effect at the Resolution of the turn it is ordered, like every stance. **A
neutral Region's own Army is always dug in**, since nobody can order it and its escape was the
accident the stance exists to end; a held Region's takes the holder's order. The computer digs in
instead of holding where a rival's Army stands next door and it has no cause to attack, and wherever
it occupies. Shown as a word on the stance row, on the roster's tooltip, as a trench line under the
Army's shield on the map, and as a Report line. A Ship is refused Dig In. `[dig_in] defence = 2` and
a `stance_dig_in` weight in every Faction block. 151 Dig In orders over the closing sweep.

## 9. A place taken whole: the destruction rolls narrowed to the Occupation that times out

*Ticket [#298](https://github.com/whaleyjoshua2/Dying-Earth/issues/298).*

**A Battle itself burns nothing.** A place that transfers by **Pacified is taken whole**; only a
transfer by the **three-turn clock** rolls a quarter per building, and **a Unique Facility rolls
like any other** (the glossary had said it never burned; the roll never honoured that, and the
glossary is corrected). The Scrubber and the Archive keep their own rules. The Moment for a place
taken by force fires on every take by force, burned or not: *Europe taken whole by the Custodians
(Pacified)*. War ppm per building stays at 2, for the closing sweep to read. The computer learns
nothing new. Two takes by force over the closing sweep, so the rule was barely exercised.

## 10. An Occupation that must be held: a break hands the place back at a cost

*Ticket [#299](https://github.com/whaleyjoshua2/Dying-Earth/issues/299).*

The march and the Carrier lift **stay legal**; the line's "may not move" was dropped at the
designer's word. But the moment an occupier's last Army leaves a place, by march, lift or death, the
Occupation breaks and **control reverts to the previous holder at +2 Unrest** (Regions only), the
occupier takes a **rung-2 offence** from that holder, the first weight-2 offence in the game, and
**the Standing the Occupation banked is wiped** (the Occupation's own gain, tracked as it runs; what
the seat held there before is kept). A neutral previous holder charges nobody. `occupation_break =
2.0` in the Unrest data, `occupation_broken_offence = 2` in the Relations data; an **Offence** entry
in the glossary names the ladder: 1 for a bid, a Smear, an Agitate or a Blockade turn; 2 for an
Occupation broken; 3 for a Battle opened or an Accord broken. No Occupation broke over the closing
sweep.

## 11. A landed Army may Attack on the turn it lands

*Ticket [#300](https://github.com/whaleyjoshua2/Dying-Earth/issues/300).*

An Army landed at a Colony its Faction does not direct **lands on Attack** and fights in a **second
ground pass after cargo**, once the orbit has been fought, so the ground is reached by whoever won
the orbit; at its own Colony it lands on Hold; the stance is inferred from whose Colony it is, with
no new field. A landing that meets no defenders **occupies the Colony the same turn**. The landing
button quotes the first-round odds. An Army still lands only at a Colony, never in a Region. The
sweep counts Armies landed; none landed in eighty games before or after the rule, so it is a
human's until the balance version. The invasion timing test moved a turn earlier: a defended Colony
changes hands in two to four turns where it took three to five.

## 12. One Army system: Levies, Standing Armies and raised Armies unified

*Ticket [#302](https://github.com/whaleyjoshua2/Dying-Earth/issues/302), added by the designer after
§7 and §8 were built.*

- **A Region's own Army** (its Standing Army) has strength and hit points of **Industry + 1 plus its
  armed steps**. While it defends it fights at that **plus 1 for a working Constabulary, plus 1
  while Unrest is under 4**, plus Dig In's two if dug in; none of that adds hit points. It **stays at
  home**: its holder can no longer march it. The row reads *strength 4, damage 0/4, defends at 5*.
- **A raised Army** is a unit of its home Region's stack: it belongs to the Region, changes hands
  with it (as it always did, since the seat it fights for is read from its home's controller), and
  **marches**. Its strength and hit points are **its home's Industry + 1 at the raise, fixed**, so a
  25-Materials Army is a 2 or a 4 by where it is raised; a save from before reads the card's 4 and 5.
  It has no defence terms of its own beyond Dig In.
- **The Levy is retired.** A threatened neutral's Standing Army gains **two armed steps for good** at
  the Income a threat appears next door, once per threat episode (a threat that leaves and returns
  arms it again); the step for holding against an attack joins the same figure; there is **no
  ceiling**. A long-neutral Region is a fortress by force, and Influence, whose threshold does not
  move, is the cheap way in. `threat_steps = 2`, `held_step = 1`.
- **A Colony's Army** belongs to the Colony, as before, and is worth the **rounded average Industry
  Level of the Regions its raising Faction holds, plus one**, fixed at the raise; a Faction holding
  nothing on Earth raises a 1. Its only defence is Dig In; a Barracks is not a police force.
- **The computer** weighs an attack against what the defenders fight at, not their bare strength,
  and raises its Armies in its highest-Industry Region.
- The Army row says strength, hit points and what it defends at; the hover names every term; the
  **Army**, **Standing Army**, **Levy**, **Strength** and **Hit Points** glossary entries say it as one
  system.

Measured on the Custodians-first seating against §7's batch: the hit points come back to Industry
+ 1 and a little of the ground war returns, three marches and two takes where there were none. No
neutral armed by threat over the closing sweep: no computer Army ever stands next to a neutral.

---

## What the closing sweep says

Run as `sweep 20 --balance --seatings --steps=300`; the output is
[`docs/dev-diary/2026-09-21-version-0.08.6/sweeps/final-0.08.6.txt`](../dev-diary/2026-09-21-version-0.08.6/sweeps/final-0.08.6.txt),
read against
[`final-0.08.5.txt`](../dev-diary/2026-09-20-version-0.08.5/sweeps/final-0.08.5.txt).

**The win column is in the header of this document.** By seating, seat 0 first: 5 / 13 / 0 / 0
against 0 / 16 / 0 / 2 with the Custodians first; 7 / 13 / 0 / 0 against 9 / 5 / 0 / 0 with the
Prospectors first (the Custodians in the second chair); 2 / 5 / 1 / 0 against 9 / 3 / 1 / 0 with the
Arkwrights first; 0 / 20 / 0 / 0 unchanged with the Archivists first. Collapses 2, 0, 12 and 0 of 20
against 2, 6, 7 and 0.

**The military block, against 0.08.5.** Over eighty games:

| | 0.08.5 | 0.08.6 |
|---|---|---|
| Battles opened | 172 | **62** |
| … on the ground (marches on held Regions) | 91 | **5** |
| Attacks in orbit | 63 | 57 |
| Armies built | 235 | 128 |
| Armies lost / Standing Armies lost | 20 / 47 | 2 / 2 |
| Warships built / lost | 129 / 0 | 228 / 1 |
| Occupations begun / broken | 67 / 2 | **2 / 0** |
| Places taken by force / by Influence | 51 / 1317 | **2** / 1428 |
| Blockade Colony-turns imposed | 157 | 172 |
| Units escaped / Battles with an escape | not counted | 5 / 5 of 62 |
| Dig In orders | none existed | 151 |
| Neutrals armed by threat / attacks held | 3 Levies / 0 | 0 / 0 |
| Armies landed at a Colony | not counted | 0 |

- **The ground war is over on the computer's board.** Five marches on a held Region in eighty
  games. The defence rules (§7, §8, §12) put a held Region's Army at Industry + 1 plus its armed
  steps, defending one or two higher, and a raised Army at its home's Industry + 1; the computer
  marches one Army at a time and its bar is 60%, so a single Army clears it against almost nobody
  now. What war is left is the warship's: 57 attacks in orbit, 172 blockade Colony-turns, and the
  computer's first warship loss.
- **The Custodians win 43 of 80.** Where the Prospectors go first they made 614 Ducats a game
  against 3580 and won 7 of 20 against 9, while the Custodians in the second chair made 1912 against
  1082 and won 13 against 5. Where the Custodians go first they won 5 against none. The Prospectors'
  war paid: without it, the seat that sells carbon credits and greenwashes nobody's Blame but its
  own is the seat with the Ducats. The Arkwrights fell from 9 to 2 and the Archivists from 2 to
  none: their wins last version came in the seatings where the Prospectors' Ducats went to war.
- **The Arkwrights-first seating runs hot**: 12 collapses of 20 against 7, at +3.01 C median where
  it was +2.89. The Custodians in the second chair build 62 Armies there and dig in 67 times, and
  the Arkwrights make 5330 Ducats a game. The other three seatings end cooler (+2.67, +2.40, +2.31
  against +2.87, +2.96, +2.19). Nothing was run with one rule alone reverted at the closing sweep;
  the per-ticket batches in the diary hold the single-rule controls.
- **Relations run warmer**: 110, 104, 126 and 104 of 240 pairs end Cold or worse against 133, 109,
  145 and 103; 45, 42, 59 and 51% carry a scar against 51, 45, 62 and 46. Fewer Battles are fewer
  offences.
- **The opening took**: every seat with a station has two Modules on it by the end of turn 3 in
  every seed, except the Prospectors where they go second, who have one (their Habitat, then a
  Trade Post later). The sweep's opening line counts a station the Arkwrights build on turn 1 as a
  starting one, since it stood at turn 1; a quirk of the counter, not of the rule.
- **Which seat the sum landed on: the Prospectors**, by the war they can no longer win and the
  Ducats that went with it, and beneath them the two seats whose few wins were that war's
  by-product. The defence was the version's aim; that it is now impassable for the computer is the
  balance version's question, already in the map's fog as the Army-appetite item: two Armies at 8
  against a 6 would clear the bar, and the computer never masses.
