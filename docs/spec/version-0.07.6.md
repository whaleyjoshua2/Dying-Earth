# Dying Earth — version 0.07.6, the second-thoughts version: a Tech pick you can change, the tutorial on the Custodians' card, the population graph off the panel, and net migration only

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.07.6](https://github.com/whaleyjoshua2/Dying-Earth/issues/172), and the
pictures that decided it are in
[`docs/dev-diary/2026-09-13-version-0.07.6/`](../dev-diary/2026-09-13-version-0.07.6/).

**What the version is.** Four small changes, three of them about letting the player change their mind
or see less of what they did not ask for. **A Tech pick is no longer locked in the instant it is
pressed**: it commits when the turn ends, which is the version's one rules change and the only thing
on it that moves a measured figure. **The tutorial moved off the title screen** onto the card of the
Faction it teaches. **The population graph came off the Climate Panel**, keeping its hover. And **the
Report stopped reporting every refugee flow**, saying one net line per Region instead, which cuts the
worst turn's refugee clutter from thirty-nine lines to fourteen.

---

## 1. A Tech pick is not locked in until the turn ends

*Ticket [#173](https://github.com/whaleyjoshua2/Dying-Earth/issues/173).*

A pick used to be final the frame it was pressed: the Shortlist was thrown away, every banked point of
Research poured into the new Tech, the seat was stamped for the next Lead tie-break, and a Tech with
enough banked behind it could complete in the middle of the Orders phase. A misclick could not be
taken back, and the Tech Tree refused a second pick.

**A human Lead's pick is now provisional.** Pressing a box records the choice and does nothing else.
Pressing another box replaces it, as often as the Lead likes, for as long as the turn lasts. **Ending
the turn commits it**, after the refusal check, so a refused turn leaves the pick as changeable as it
was. Everything that used to happen at the press happens at the commit: the Shortlist is thrown away,
the banked Research pours in under each seat's own name, the log line is written, and only then may
the Tech complete -- so a Tech finished by banked Research lands with the turn's other Moments instead
of arriving mid-Orders.

**The Shortlist is kept while the pick can change.** Redrawing the three on every change would be a
free reroll.

**A computer seat commits in the same breath it picks**, since it never changes its mind and the loop
that finishes a Tech expects the next one standing. No computer player's behaviour changes.

**The Archivists' Provisional Findings reads the Tech frozen at the head of the turn**, not whatever
is under research this instant, so a Lead changing its mind cannot re-price orders already placed. A
pick made this turn is read from the next one. This is the one rule on the map that moves a measured
figure; section 6 says by how much.

**On the Tech Tree**, a box chosen this turn but not yet committed wears a paler amber than the
settled one, reads `chosen`, and loses its own `Pick` button. The legend carries the new colour beside
the settled amber. Above the tree, in the Lead's own colour: *Chosen for this turn. Press another box
to change it; it is locked in when the turn ends.*

**The headless `play` driver** reads the pick too, and says so: *"Deep Mining IS CHOSEN FOR THIS TURN,
and not locked in until the turn ends. Another `tech <name>` line changes it."* Two `tech` lines in one
turn are both accepted and the second wins.

A save written before this version carries a Tech that is already committed.

---

## 2. The tutorial is asked for on the Custodians' card

*Ticket [#174](https://github.com/whaleyjoshua2/Dying-Earth/issues/174).*

The title screen's **Tutorial button is retired**, and New Game is the top of the list again. The
tutorial is asked for by a tick at the foot of the Custodians' card on the Faction screen, reading
**`Play Tutorial`**, with a hover saying what it does.

**A ticked card still chooses a start.** The tick is on the Faction, not on the whole opening, so
`Play the Custodians` goes on to the start screen as it always did and the notes begin once a Region
has been chosen. None of the five notes names a Region, so any start reads correctly.

**The other three cards say nothing about it.** The tick is remembered while the Faction screen is
open and cleared when a game begins. A player who ticks it and then plays another Faction gets no
notes: every note is written about the Custodians.

---

## 3. The population graph comes off the Climate Panel

*Ticket [#175](https://github.com/whaleyjoshua2/Dying-Earth/issues/175).*

The population history is drawn in one place now, the top bar's **Population hover**. It leaves the
Climate Panel.

**The Emissions history stays on the panel.** The panel is the page about emissions and that chart is
the page's own subject over time, where the population chart was a guest. Nothing fills the freed
space, the Emissions chart keeps the height it had, and the growth-rate line is unchanged.

---

## 4. The Report says only net migration, and only when migration happens

*Ticket [#176](https://github.com/whaleyjoshua2/Dying-Earth/issues/176).*

The Report used to speak once for every refugee flow: one line per Region per cause that drove people
out, and one more per Region that received any. A Region that lost people to the sea and to the heat
spoke twice, and one that took ten people and sent ten away spoke twice while netting nothing.
Measured over ten computer-played games, the worst turn spent **39 of its 74 Report lines** on
refugees, the median turn 12, and refugees were **30% of the median Report**.

**The Report now says one net line per Region**, and only where the net is worth at least **half a
person** -- `report_net_floor` in `unrest.toml`, the same figure Unrest itself rounds by.

- Gained: `Russia took in 3.0 people; Unrest rose by 1 to 4.`
- Gained, with some of it cancelled by what left: `Russia took in 3.0 people of 7.0 arriving; Unrest
  rose by 1 to 4.`
- Gained too little to move Unrest: `Russia took in 3.0 people.`
- Lost, to two or more causes: `China lost 4.0 people to its neighbours: mostly the heat.`
- Lost, to one: `Russia lost 3.0 people to its neighbours: the reefs.`
- Flows that cancel: nothing at all.

**Unrest is still charged on the gross arrivals**, which is unchanged: a Region that takes ten people
and sends ten away has still absorbed ten people's worth of grievance. Where the gross differs from
the net the line names both, because that clause is the one place the Report explains an Unrest rise.

**The largest cause survives** into a Region's loss line. A Region's departures are counted by cause
in `refugees_out`, beside the arrivals counter that already existed.

**The game's log is untouched**: it keeps one line per flow, naming where the people went and why.

After the change, over the same ten games: the worst turn spends 14 of 29 lines, the median turn
**none**, and the median Report carries **no** refugee line at all.

---

## 5. What was measured

- **The suite is 247 tests**, clippy clean with `-D warnings`. Two are new: one pins the Tech pick's
  whole rule end to end, one pins the net migration rule in five cases. Four older tests were repinned
  with their reasons written beside them.
- **Every new rule was witnessed red before it was believed green**: reverted on purpose, the test
  watched to fail, the rule restored.
- **A whole game was played from seat 0 headlessly** through the `play` driver, seed 11, Custodians
  from Europe, giving no order but the owed Tech picks and one deliberate change of pick on turn 1:
  27 turns, **none refused**, and the world collapsed on turn 27 at +3.0 C.

---

## 6. The sweep, and the one figure that moved

Twenty seeds in each of four seatings, against the 0.07.5 baseline.

| seat 0 | wins | Collapses | draws | tree completes |
|---|---|---|---|---|
| Custodians | Archivists 8, Custodians 1 | **11** | 0 | 19 of 20 |
| Prospectors | Archivists 11, Custodians 3 | 6 | 0 | 19 of 20 |
| Arkwrights | Archivists 11, Custodians 8, Arkwrights 1 | 0 | 0 | 7 of 20 |
| Archivists | Archivists 16, Custodians 4 | 0 | 0 | **4 of 20** |
| **totals** | **Archivists 46**, Custodians 16, Arkwrights 1, Prospectors 0 | **17** | **0** | **49 of 80** |

**Every win and every Collapse reproduces the baseline to the cell.** One figure moved: the tree
completes, 50 of 80 to **49**, in the Archivist seating.

It was run down rather than accepted. The seed is **3 of the Archivist seating**, which finished 15
Techs and ended on turn 21 where 0.07.5 finished 17 and ended on turn 23; the other seventy-nine games
are identical, tech for tech. Reverting the Provisional Findings freeze in a worktree built from
0.07.5 restores that seed exactly. **The cause is the freeze decided on ticket #173**: the Archivists
no longer get half the effect of a Tech picked mid-turn until the turn after, and in one game of
eighty that cost two Techs. It is the decided rule doing what it says, not a defect, and no Faction's
win count moved.

---

## 7. The kit

The **Windows kit** only: `dying-earth.exe` built with a statically linked CRT, `assets/`, and the
playtest note as `README.txt`, zipped into `dist/dying-earth-0.07.6-playtest.zip`. `dist/` is
gitignored, so the kit is an artifact on the machine and not a commit.

---

## 8. Found on the way, and not fixed here

[The last Report of every game is dated January 2030](https://github.com/whaleyjoshua2/Dying-Earth/issues/178):
`end_turn` clears the Report for the coming turn and returns on a finished game before the turn is
stamped on it, so every game's last Report carries turn 0 in its heading. Found while photographing a
hot turn's Report for section 4, measured in ten games out of ten, and filed as a defect rather than
smuggled into this version.
