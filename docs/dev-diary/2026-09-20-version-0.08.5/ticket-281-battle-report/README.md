# The Battle Report: Battles by name, as a line and as a Moment

Ticket [#281](https://github.com/whaleyjoshua2/Dying-Earth/issues/281) on
[map #275](https://github.com/whaleyjoshua2/Dying-Earth/issues/275).

Both pictures are `shot: battle:1 menus:1`, 1280x800: the Report after four stacks fought in Mars
orbit on Attack. The dice differ between runs, so the two show the two cases.

| picture | what it shows |
|---|---|
| [`report-a-ship-lost.png`](report-a-ship-lost.png) | **A unit died.** Under *In space*: *"Battle at Mars orbit: the Custodians and the Prospectors and the Arkwrights and the Archivists attacked at 50% first-round odds; PMV Magellan destroyed."* The Battle Report block names every unit: *TSV Valiant escaped after 3 hits; TSV Vanguard took 1 hit*, each party with the odds it attacked at. The headline in this picture is *Prospectors PMV Magellan destroyed (battle)*, the Ship's own line: it was taken before the ordering fix below, which puts the Battle's line first among a turn's rank-4 lines so that it headlines instead. |
| [`report-nobody-lost.png`](report-nobody-lost.png) | **Nobody died.** The same Battle with kinder dice: *"… attacked at 50% first-round odds; nobody lost a unit."* files under *In space* and the headline is the turn's Methane Burst, because a bloodless Battle is unranked and never reads over an Event or a Break. The block: *TSV Vanguard escaped after 2 hits; PMV Indomitable escaped after 3 hits; ACV Challenger took 0 hits (strength 0)*. |

No batch was run: nothing about play changed; the computer seats read no Report.

## What was decided, in the designer's words

- *"q1 a"* -- **every Battle is a line** at its place, ranked with a Ship destroyed when a unit died
  or a place changed hands, unranked otherwise; the block at the foot stays as the detail.
- *"q2 a"* -- the **Moment fires when any unit is destroyed**, an Army included, and names it; the
  burned-building case keeps a Moment under its own name, **A place taken by force**.
- *"q3 a"* -- **names**: the party line reads the Ship's and the Army's names; a neutral Region's
  Army reads *the 1st Egyptian Army*.
- *"q4 a"* -- **who was there and what each lost**: *TSV Valiant took 2 hits; PMV Aurora escaped*.
- *"q5 yes"* -- the Battle record gains a **real place**, saved with a default, and the line jumps
  to it.
- *"q6 yes"* -- the line says the **first-round odds** the aggressor faced, labelled as such; the
  button's own wording stays in the map's fog.

## Settled by the builder, to be corrected if wrong

- **Two line kinds.** A Battle that cost a unit writes the existing rank-4 kind; a bloodless one
  writes a new unranked kind, filed by its place (a Body or Colony under *In space*, a Region under
  *On Earth*).
- **The Battle's line goes in before the losses are applied**, so among a turn's rank-4 lines it is
  the earliest and headlines over the Ship's or Army's own line. The first picture predates this.
- **An Army destroyed writes a line by name**, as a Ship has since 0.08.1: *"the 1st Egyptian Army
  (neutral) destroyed (battle)."*, or *"(lost with its Carrier)"* when its Carrier went down.
- **Several attackers** are named together on the line; the odds quoted are the first attacker's.
- **The party text** reads *took N hits*, *escaped after N hits* or *destroyed*, per unit, in the
  order the units stood; the old destroyed and escaped lists stay on the record for the log.
- **The Moment kinds are ten**, and the Moments corner lists the new one; every old save loads,
  since the record's place and odds default to none.

## Witnessed red

The test was watched to fail with the Battle line switched off, at *"a Report line for the
Battle"*, and passed on the restore. The 0.08.4 rival-Moment test moved its count of Moment kinds
from nine to ten. `341 passed`, `6 passed`, clippy clean with `-D warnings`.
