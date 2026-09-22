# Dying Earth — version 0.08.7, the presentation version: End Turn in its own column, eight duplicated texts cut, an offline building shown offline, the Prospectors' Fund off the top bar, an odds hover that names the defender, a threat line beside the challenger line, last turn's Battle drawn on the map, the Army orders in the Armies list, and stance words that say what they do

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.08.7](https://github.com/whaleyjoshua2/Dying-Earth/issues/304), and the
pictures that decided it are in
[`docs/dev-diary/2026-09-21-version-0.08.7/`](../dev-diary/2026-09-21-version-0.08.7/).

**What the version is.** Nine pieces of interface and not one rule. Five are the presentation
review's proposals of 2026-09-19, taken nearly word for word: an odds hover that names the defender
(§5), a military threat line beside the Influence challenger line (§6), last turn's Battle drawn on
the map (§7), the Army orders moved up the card into the Armies list (§8), and stance words that say
what they do on hover (§9). Beside them, four pieces of housekeeping: End Turn in its own column of
the command cluster (§1), eight duplicated texts cut (§2), an offline building shown offline in its
box (§3), and the Prospectors' Fund figure taken off the top bar (§4). No figure moved, no computer
seat's reasoning changed, and nothing was added to the save.

**What it did to the win column**, 20 seeds across four seatings at the shipped climate cell
(sink 6, step 300), against the 0.08.6 baseline: nothing, line for line; see *What the closing
sweep says* at the foot.

The designer, shown the unchanged column and the pictures: **ship**, and the version ships as the
**Windows kit alone**, built on this machine (`dist/dying-earth-0.08.7-windows.zip`); the
designer's word on the closing ticket was *"ship and windows only"*. No Linux kit was built.

---

## 1. End Turn in its own column at the right of the command cluster, at the bottom

*Ticket [#305](https://github.com/whaleyjoshua2/Dying-Earth/issues/305).*

The command cluster is **two columns**. The left column holds the four rows as 0.08.6 left them:
the Allotment line, the spend rail at the column's full width, the Spend button, and Max with the
every-turn tick. The right column holds the **End Turn sun alone**, at the sun's own width and
nothing more, bottom-aligned so its lower edge sits level with the Max row, with nothing above it.
The strip's height is whatever the taller column needs. The Enter key, the hover, the embers while a
Tech pick is owed, and the spectator's rectangle on the top bar are unchanged. The left column wraps
its text: at the side panel's default width the *Click a Region or a Colony* sentence was wider than
the column and pushed the sun off the panel's edge, which the first picture caught.

## 2. Eight duplicated texts cut

*Ticket [#306](https://github.com/whaleyjoshua2/Dying-Earth/issues/306).*

Thirteen places where the game said a thing twice were listed on the ticket; the designer cut
eight. **Gone**: the command cluster's *Click a Region or a Colony to spend on it* label (a greyed
Spend button keeps the strip's shape; the Max button's hover keeps the sentence); the Climate
Panel's standing sentence under the Blame block, a clause of the heading's hover word for word; the
Colony card's text lines for queued Modules, which their hatched tiles already carry; the offline
suffix on a building's row, kept on the box's hover; the Agitate hover's last sentence restating the
Unrest thresholds; the Victory window's Victory Condition sentence over each seat's two labelled
bars (the Rulebook keeps its copy); the Faction window's Victory progress lines and bars, a copy of
the Victory window's (the history chart stays); and the hover signposts, *(hover a button for what
it does)* and *(hover a button for what it makes)* off the Orders and Build here headings, *Hover a
box for its effect.* off the Tech Tree, and *Hover a figure for the building-by-building breakdown.*
off the Faction window, which supersedes spec 0.07.4's line naming the Orders heading. The
rival's-page line *A rival's income is shown as totals only* stays, since it states a rule.

**Kept**, as figures that belong to two views: the Smear and Greenwash captions, the Influence
figure on the top bar and in the cluster, last turn's income on the top bar and the Faction window,
the Facilities heading's slot count, and End Turn's hover, which carries the key.

## 3. An offline building shown offline in its box

*Ticket [#307](https://github.com/whaleyjoshua2/Dying-Earth/issues/307).*

A standing building that is not mothballed and is not online -- **short of Energy** by the
shortfall rule, **struck by a card** until the next Resolution, in a **grid-failed** Colony, or an
Occupied Colony's **Archive** -- takes the mothballed tile's dimming (the darker fill, the picture
at four tenths) and the word **offline** in its bottom-left corner, in a warm tan against
mothballed's cool grey; no hatch, no red edge. Its hover names the cause: *short of Energy*, *until
the next Resolution, struck by a card* (the card's name is not stored), *the grid is down*, or
*while the Colony is Occupied*. Until this ticket such a building was drawn exactly like a working
one and only the hover said *offline, making nothing*, never why. A blockaded Colony's Modules,
which make nothing and still pay by rule, and a Facility in a Region nobody directs, which is idle,
are not offline and keep their looks. The roster and the row under the grid are unchanged. The
glossary gains an **Offline** entry.

## 4. The Prospectors' Fund figure off the top bar

*Ticket [#308](https://github.com/whaleyjoshua2/Dying-Earth/issues/308).*

The *Fund N (S%)* figure, which ticket #72 put inside the top bar's Materials group in 0.05.5 when
the Fund held Materials and which ticket #240 left there when the Fund began to hold Ducats, is
**gone from the top bar**. It was first moved beside Ducats as a progress bar in the Prospectors'
colour, filled to the Fund's share of its 2500 with its figures on the hover, at the designer's word
(*"move besides duckets ... bar only details on hover"*); shown it, the designer cut it: *"let's just
cut it, the archivist bank is not shown on the bar."* No Faction's fund is on the top bar; the
Prospectors read theirs on the Victory window, which carries the progress line, the slider's heading
and the Fund line, and on the Faction window's chart. The Prospectors' figures row now fits a
1280-pixel window whole.

## 5. An odds hover that names the defender

*Ticket [#309](https://github.com/whaleyjoshua2/Dying-Earth/issues/309).*

The march buttons on a Region card read **attack Russia** and **move to Russia**, with no
percentage on the face; the figure is on a hover. An attack button's hover, within the six-line
rule: the place and its holder (*Russia: neutral*, *held by the Prospectors*, *occupied by the
Prospectors*); one line per defender with its name, its strength, what it defends at (dug in noted)
and its damage, up to three, and past three two named and the rest counted; *N% is the chance to
win the first exchange: your A against their D.*; and *Moving costs nothing; the Army arrives on
Attack.* An undefended place says so and that the Occupation begins. A move into a Region the
player holds hovers two lines: held by you, the move free, the Army keeping its stance (the
Resolution sets Attack only on a Region that is not the mover's). The landing button on a Ship stack
card, *Land the Army to attack X*, carries the same hover worded for a landing; at the player's own
Colony, *lands on Hold*. The figure is the first exchange, labelled so; a whole-Battle estimate is
the balance version's question. The AI reads the same odds function as before.

## 6. A threat line beside the challenger line on a held Region

*Ticket [#310](https://github.com/whaleyjoshua2/Dying-Earth/issues/310).*

Under the challenger line in a held Region's Standings block, a **threat line**: the rival raised
Army standing in a neighbouring Region with the best first-exchange odds against this Region's
defenders (ties to the strongest), named with its strength and the Region it stands in -- *The
Prospectors' 2nd Russian Army, strength 3, stands next door in Russia.* -- and **never with its
stance**, consistent with #282's rule that a Region arms stance-blind. In the rival's colour; amber,
with *Dig In here to hold it.*, when their odds reach the bar the computer seats attack at
(`attack_odds` in `ai.toml`, 0.6), which is the computer's habit and not a rule of the game, and the
hover says so. The hover carries their strength against this Region's defended strength and the
odds. No line at all when nobody stands next door. Regions only: a Colony has no neighbours. The
full Region name; no short-name field. The rule is one engine helper, `nearest_army_threat`, beside
`nearest_challenger`, tested and watched to fail first. The glossary gains **Challenger line** and
**Threat line** entries.

## 7. Last turn's Battle drawn on the map, and the shields that fought

*Ticket [#311](https://github.com/whaleyjoshua2/Dying-Earth/issues/311).*

For the one Orders phase the Battle record lives (the Report is wiped at End Turn and filled by the
Resolution, so nothing is added to the save), a **ring in the aggressor's colour** round a Region's
label on Earth and round a Colony's label on its Body's surface view; its hover reads the record
(who attacked, the rounds, each party's line) and a click on the ring's crown opens the Report, the
crown so the label's own click still selects the place. **No ring round a Body on the Solar map**,
at the designer's word, so a Battle in orbit is marked nowhere on the map; last version's Battles
were mostly in orbit. Every **shield** at the place is **outlined in the aggressor's colour**, since
a Battle is a melee of every party present and nothing records which Army fought; and a shield whose
stack carries damage wears a **red pip**, which means hurt rather than fought and stays until the
damage heals.

## 8. The Army orders moved into the Armies list, a tenth larger

*Ticket [#312](https://github.com/whaleyjoshua2/Dying-Earth/issues/312).*

On the Region card and the Colony card alike, the **Armies block holds the Army orders**: the stance
row directly under the *Armies* heading, since it is per place; one row per Army; and under each of
the player's raised Armies its march buttons and repairs, indented under the row. The *Army orders*
block at the foot of the Region card, below the fold at 1080 in the presentation review's picture,
and the Colony card's block after its Orders section are gone; the Colony card gains an *Armies*
heading it never had, drawn only when an Army is there. The whole block is **a tenth larger** at
one named constant, `ARMY_LIST_SCALE = 1.1`, applied to the block's text styles as the command
cluster applies its own, since nothing in the block had a size of its own to multiply. *Stance:*
stays as the row's caption.

## 9. Stance words that say what they do on hover

*Ticket [#313](https://github.com/whaleyjoshua2/Dying-Earth/issues/313).*

Each stance has **one sentence**, kept in the engine beside its name (`Stance::one_liner`): Attack
*strikes at the place it is sent to, or fights where it stands*; Hold *stands and fights where it
is; does nothing of its own*; Evade *avoids battle where it can: an even chance to slip away before
the first exchange*; Dig In *fights two stronger in defence and never disengages, and cannot march
or board a Carrier until its stance is changed and the turn has passed*; Intercept, for Ships,
*fights what arrives this turn, before it can land*; Blockade, for Ships, *shuts this orbital slot
to every other Faction: no landing, no refuel, no building in it*. An Army handed a Ship's stance,
or the reverse, says it holds instead. **Every label on the stance row hovers** *Name: sentence* and
the line every stance shares, *A stance persists until it is changed.*, with no marker (#233). The
Army roster row reads the same sentence between its own two; a **Ship's roster row gains its stance
word** in brackets and the hover, so a Ship on Intercept, the case nobody could read anywhere, is
readable in two places. *Stance:* stays.

---

## Shot aids added for the pictures

Building aids, not rules: `offline:1` puts a Facility and a Module of seat 0's offline as a
shortfall would; `army:1` raises an Army of seat 0's in its start state; `threat:1` hands the first
neighbour of seat 0's start state to seat 1 and raises an Army there; `battle:region` does that and
marches seat 0's raised Army on it, running the turn so the Report carries a ground Battle.

## What the closing sweep says

Run as `sweep 20 --balance --seatings --steps=300`; the output is
[`docs/dev-diary/2026-09-21-version-0.08.7/sweeps/final-0.08.7.txt`](../dev-diary/2026-09-21-version-0.08.7/sweeps/final-0.08.7.txt),
read against
[`final-0.08.6.txt`](../dev-diary/2026-09-21-version-0.08.6/sweeps/final-0.08.6.txt).

**It is the same file, line for line, ignoring line endings.** No rule, figure or computer weight
moved on this map, the seeds are the seeds, and the eighty games played out move for move as they
did at the 0.08.6 close: the win column stands at **Custodians 43, Prospectors 21, Arkwrights 2,
Archivists 0**, collapses **14 of 80** (2, 0, 12 and 0 by seating), 62 Battles of which 57 in orbit,
5 marches on a held Region, 2 Occupations begun, 2 places taken by force. That is the expected
result and the check the map asked for: had the column moved, something on this map had drawn more
than it should. The balance is the next version's, with this sweep, which is 0.08.6's, as its
baseline.

**The test suite**: 353 engine tests and the rest all green, clippy clean across the workspace with
warnings denied, `GAME_VERSION` 0.08.7 with `SAVE_VERSION` unchanged, so a 0.08.6 save loads.
